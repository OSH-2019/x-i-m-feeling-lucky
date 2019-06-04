use std::io;
use std::path::Path;
use std::mem::size_of;
use std::cmp::min;

use util::SliceExt;
use mbr::MasterBootRecord;
use vfat::{Shared, Cluster, File, Dir, Entry, FatEntry, Error, Status};
use vfat::{BiosParameterBlock, CachedDevice, Partition};
use traits::{FileSystem, BlockDevice};

#[derive(Debug)]
pub struct VFat {
    device: CachedDevice,
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    sectors_per_fat: u32,
    fat_start_sector: u64,
    data_start_sector: u64,
    root_dir_cluster: Cluster,
}

impl VFat {
    pub fn from<T>(mut device: T) -> Result<Shared<VFat>, Error>
        where T: BlockDevice + 'static
    {
        let mbr = MasterBootRecord::from(&mut device)?;
        let partition = &mbr.partition_entries[0];
        if !partition.partition_type.is_fat() {
            return Err(Error::NotFound);
        }

        let start = partition.relative_sector as u64;
        let ebpb = BiosParameterBlock::from(&mut device, start)?;

        let dev = CachedDevice::new(
            device,
            Partition {
                start,
                sector_size: ebpb.bytes_per_sector() as u64,
            }
        );

        Ok(Shared::new(Vfat {
            device: dev,
            bytes_per_sector: ebpb.bytes_per_sector,
            sectors_per_cluster: ebpb.sectors_per_cluster,
            sectors_per_fat: ebpb.sectors_per_fat,
            fat_start_sector: &start + ebpb.reserved_sectors as u64,
            data_start_sector: &start + ebpb.reserved_sectors as u64
                + ebpb.sectors_per_fat_assist as u64 * ebpb.num_of_fat as u64,
            root_dir_cluster: Cluster::from(ebpb.root_dir_cluster_number),
        }))
    }


    // TODO: The following methods may be useful here:
    //
    //  * A method to read from an offset of a cluster into a buffer.
    //
    fn read_cluster(
        &mut self,
        cluster: Cluster,
        offset: usize,
        buf: &mut [u8],
    ) -> io::Result<usize> {
        Ok(0)
    }

//      * A method to read all of the clusters chained from a starting cluster
//        into a vector.

    fn read_chain(
        &mut self,
        start: Cluster,
        buf: &mut Vec<u8>,
    ) -> io::Result<usize> {
        Ok(0)
    }

//      * A method to return a reference to a `FatEntry` for a cluster where the
//        reference points directly into a cached sector.

    fn fat_entry(&mut self, cluster: Cluster) -> io::Result<&FatEntry> {
        let a: FatEntry = FatEntry(0);
        Ok(&a)
    }
}

impl<'a> FileSystem for &'a Shared<VFat> {
    type File = ::traits::Dummy;
    type Dir = ::traits::Dummy;
    type Entry = ::traits::Dummy;

    fn open<P: AsRef<Path>>(self, path: P) -> io::Result<Self::Entry> {
        use vfat::Entry;
        use std::path::Component;

        let mut cur_dir = Entry::Dir(Dir::root(self.clone()));

        for x in path.as_ref().components() {
            match x {
                Component::Normal(name) => {
                    cur_dir = cur_dir.as_dir().ok_or(io::Error::new(io::ErrorKind::NotFound,"Not Found"))?.find(name)?
                },

                Component::ParentDir => {
                    cur_dir = cur_dir.as_dir().ok_or(io::Error::new(io::ErrorKind::NotFound,"Not Found"))?.find("..")?
                }

                _ => {},
            }
        }
    }

    fn create_file<P: AsRef<Path>>(self, _path: P) -> io::Result<Self::File> {
        unimplemented!("read only file system")
    }

    fn create_dir<P>(self, _path: P, _parents: bool) -> io::Result<Self::Dir>
        where P: AsRef<Path>
    {
        unimplemented!("read only file system")
    }

    fn rename<P, Q>(self, _from: P, _to: Q) -> io::Result<()>
        where P: AsRef<Path>, Q: AsRef<Path>
    {
        unimplemented!("read only file system")
    }

    fn remove<P: AsRef<Path>>(self, _path: P, _children: bool) -> io::Result<()> {
        unimplemented!("read only file system")
    }
}
