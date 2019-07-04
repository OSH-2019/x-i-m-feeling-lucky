use std::io;
use std::io::Write;
use std::mem;
use std::path::Component;
use std::path::{Path, PathBuf};

use mbr::MasterBootRecord;
use traits::{BlockDevice, FileSystem};
use util::SliceExt;
use vfat::{BiosParameterBlock, CachedDevice, Partition};
use vfat::{Cluster, Dir, Entry, Error, FatEntry, File, Shared, Status};

#[derive(Debug)]
pub struct VFat {
    pub device: CachedDevice,
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub sectors_per_fat: u32,
    pub fat_start_sector: u64,
    pub data_start_sector: u64,
    pub root_dir_cluster: Cluster,
}

impl VFat {
    pub fn from<T>(mut device: T) -> Result<Shared<VFat>, Error>
    where
        T: BlockDevice + 'static,
    {
        let mbr = MasterBootRecord::from(&mut device)?;

        let start = mbr.first_fat32().ok_or(Error::NotFound)?.relative_sector as u64;
        let ebpb = BiosParameterBlock::from(&mut device, start)?;

        let dev = CachedDevice::new(
            device,
            Partition {
                start,
                sector_size: ebpb.bytes_per_sector as u64,
            },
        );

        Ok(Shared::new(VFat {
            device: dev,
            bytes_per_sector: ebpb.bytes_per_sector,
            sectors_per_cluster: ebpb.sectors_per_cluster,
            sectors_per_fat: ebpb.sectors_per_fat() as u32,
            fat_start_sector: start + ebpb.reserved_sectors as u64,
            data_start_sector: start
                + ebpb.reserved_sectors as u64
                + ebpb.sectors_per_fat() as u64 * ebpb.num_of_fat as u64,
            root_dir_cluster: Cluster::from(ebpb.root_dir_cluster_number),
        }))
    }

    //  * A method to read from an offset of a cluster into a buffer.
    fn read_cluster(
        &mut self,
        cluster: Cluster,
        offset: usize,
        mut buf: &mut [u8],
    ) -> io::Result<usize> {
        let sector_start = self.data_start_sector as usize
            + (cluster.cluster_index() as usize - 2usize) * self.sectors_per_cluster as usize;

        let mut already_read: usize = 0;

        loop {
            let sector_index = (offset + already_read) as usize / self.bytes_per_sector as usize;
            if sector_index >= self.sectors_per_cluster as usize {
                break;
            } else {
                let new_offset = (offset + already_read) as usize
                    - sector_index as usize * self.bytes_per_sector as usize;
                let newly_read = buf.write(
                    &(self.device.get(sector_start as u64 + sector_index as u64)?)[new_offset..],
                )?;
                already_read += newly_read;
                if buf.is_empty() {
                    break;
                }
            }
        }

        Ok(already_read)
    }

    //  * A method to read all of the clusters chained from a starting cluster
    //    into a vector.
    pub fn read_chain(&mut self, start: Cluster, buf: &mut Vec<u8>) -> io::Result<usize> {
        let mut already_read: usize = 0;
        let mut current = start;

        loop {
            let buf_len = buf.len();
            buf.resize(
                buf_len + self.bytes_per_sector as usize * self.sectors_per_cluster as usize,
                0,
            );

            already_read += self.read_cluster(current, 0, &mut buf[already_read..])?;

            let fat_entry = self.fat_entry(current)?.status();

            match fat_entry {
                Status::Data(next) => {
                    current = next;
                }

                Status::Eoc(_) => {
                    return Ok(already_read);
                }
                _ => {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid sector"));
                }
            }
        }
    }

    //  * A method to return a reference to a `FatEntry` for a cluster where the
    //    reference points directly into a cached sector.
    fn fat_entry(&mut self, cluster: Cluster) -> io::Result<&FatEntry> {
        let cluster_index = cluster.cluster_index();
        let entries_per_sector = self.bytes_per_sector / mem::size_of::<FatEntry>() as u16;

        let fat_index = cluster_index / entries_per_sector as usize;
        let fat_offset = cluster_index % entries_per_sector as usize;

        if fat_index >= self.sectors_per_fat as usize {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "F***ing Invalid Cluster",
            ));
        }

        let data = self.device.get(self.fat_start_sector + fat_index as u64)?;
        let entries: &[FatEntry] = unsafe { data.cast() };
        Ok(&entries[fat_offset])
    }
}

impl<'a> FileSystem for &'a Shared<VFat> {
    type File = File;
    type Dir = Dir;
    type Entry = Entry;

    fn open<P: AsRef<Path>>(self, path: P) -> io::Result<Self::Entry> {
        use traits::Entry;
        use vfat::Entry as Entry2;

        let mut current_dir = Entry2::Dir(Dir::root(self.clone()));

        for x in path.as_ref().components() {
            match x {
                Component::Normal(name) => {
                    current_dir = current_dir
                        .as_dir()
                        .ok_or(io::Error::new(io::ErrorKind::NotFound, "File not found"))?
                        .find(name)?
                }
                Component::CurDir => {
                    current_dir = current_dir
                        .as_dir()
                        .ok_or(io::Error::new(io::ErrorKind::NotFound, "File not found"))?
                        .find(".")?
                }
                Component::ParentDir => {
                    current_dir = current_dir
                        .as_dir()
                        .ok_or(io::Error::new(io::ErrorKind::NotFound, "File not found"))?
                        .find("..")?
                }
                _ => {}
            }
        }

        Ok(current_dir)
    }

    fn create_file<P: AsRef<Path>>(self, _path: P) -> io::Result<Self::File> {
        // Skip this as this is a read only filesystem
        //        Ok(Self::File::new())
        panic!("Dummy")
    }

    fn create_dir<P>(self, _path: P, _parents: bool) -> io::Result<Self::Dir>
    where
        P: AsRef<Path>,
    {
        // Skip this as this is a read only filesystem
        //        Ok(Self::Dir::new())
        panic!("Dummy")
    }

    fn rename<P, Q>(self, _from: P, _to: Q) -> io::Result<()>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        // Skip this as this is a read only filesystem
        panic!("Dummy")
    }

    fn remove<P: AsRef<Path>>(self, _path: P, _children: bool) -> io::Result<()> {
        // Skip this as this is a read only filesystem
        panic!("Dummy")
    }

    /// This is a fake canonicalize, it just simplify the path
    fn canonicalize<P: AsRef<Path>>(self, path_ref: P) -> io::Result<PathBuf> {
        let mut ret = PathBuf::new();
        for item in path_ref.as_ref().components() {
            match item {
                Component::RootDir => ret = PathBuf::from("/"),
                Component::ParentDir => {
                    ret.pop();
                }
                Component::Normal(thing) => ret.push(thing),
                Component::CurDir => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "path must be absolute",
                    ));
                }
                Component::Prefix(_) => {
                    unimplemented!("Component::Prefix should only appear in Windows")
                }
            }
        }

        Ok(ret)
    }
}
