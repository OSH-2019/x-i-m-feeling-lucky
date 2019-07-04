use std::ffi::OsStr;
use std::io;
use std::str;
use std::string::String;
use std::vec::IntoIter;

use util::VecExt;

use traits;

use vfat::{VFat, Shared, File, Cluster, Entry};
use vfat::{Metadata, Attributes, Timestamp, Time, Date};

#[derive(Debug)]
pub struct Dir {
    pub name: String,
    //    pub lfn: String,
    pub first_cluster: Cluster,
    pub vfat: Shared<VFat>,
    pub metadata: Metadata,

}

impl Dir {
    pub fn root(vfat: Shared<VFat>) -> Dir {
        Dir {
            name: String::from("/"),
            first_cluster: vfat.borrow().root_dir_cluster,
            vfat: vfat.clone(),
            metadata: Metadata::default(),
        }
    }
}


#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct VFatRegularDirEntry {
    name: [u8; 8],
    extension: [u8; 3],
    attributes: Attributes,
    reserved_by_windows_nt: u8,
    creation_time_in_tenths: u8,
    create_time: Time,
    create_date: Date,
    access_data: Date,
    first_cluster_number_high: u16,
    modify_time: Time,
    modify_data: Date,
    first_cluster_number_low: u16,
    size_in_bytes: u32,

}

impl VFatRegularDirEntry {
    pub fn metadata(&self) -> Metadata {
        Metadata {
            attributes: self.attributes,

            created: Timestamp {
                date: self.create_date,
                time: self.create_time,
            },
            accessed: Timestamp {
                date: self.access_data,
                time: Time::new(),
            },
            modified: Timestamp {
                date: self.modify_data,
                time: self.modify_time,
            },
        }
    }
}


#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct VFatLfnDirEntry {
    sequence_number: u8,
    name_characters: [u16; 5],
    attributes: Attributes,
    lfn_type: u8,
    checksum: u8,
    name_characters_second: [u16; 6],
    always_zero: u16,
    name_characters_third: [u16; 2],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct VFatUnknownDirEntry {
    info: u8,
    _placeholder0: [u8; 10],
    attributes: Attributes,
    _placeholder1: [u8; 20],
}

pub union VFatDirEntry {
    unknown: VFatUnknownDirEntry,
    regular: VFatRegularDirEntry,
    long_filename: VFatLfnDirEntry,
}

impl Dir {
    /// Finds the entry named `name` in `self` and returns it. Comparison is
    /// case-insensitive.
    ///
    /// # Errors
    ///
    /// If no entry with name `name` exists in `self`, an error of `NotFound` is
    /// returned.
    ///
    /// If `name` contains invalid UTF-8 characters, an error of `InvalidInput`
    /// is returned.
    pub fn find<P: AsRef<OsStr>>(&self, name: P) -> io::Result<Entry> {
        use traits::{Dir, Entry};
//        let name_str = match name.as_ref().to_str() {
//            Some(x) => {
//                x
//            }
//            _ => {
//                return Err(io::Error::new(io::ErrorKind::InvalidInput, "InvalidInput"));
//            }

        let name_str = name.as_ref().to_str().ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Invalid Input"))?;

        self.entries()?.find(|item| {
            item.name().eq_ignore_ascii_case(name_str)
        }).ok_or(io::Error::new(io::ErrorKind::NotFound, "Not Found"))
    }
}



pub struct VFatIterator {
    entries: IntoIter<VFatDirEntry>,
    vfat: Shared<VFat>,
}

impl Iterator for VFatIterator {
    // I just can't do this. So I "copy from Github". Orz
    type Item = Entry;
    fn next(&mut self) -> Option<Self::Item> {
        let mut lfn_vec = [0u16; 13 * 31]; // Max lfn length = 13 u16 * 31 entries
        let mut has_lfn = false;

        for ref entry in self.entries.by_ref() {
            let unknown_entry = unsafe { entry.unknown };
            match unknown_entry.info {
                0x00 => {
                    return None;
                }
                0xE5 => {
                    continue;
                }
                _ => {

                }
            }

            if unknown_entry.attributes.lfn() {
                let entry = unsafe { entry.long_filename };
                has_lfn = true;
                let seq = (entry.sequence_number & 0x1F) as usize - 1;
                lfn_vec[seq * 13..seq * 13 + 5].copy_from_slice(&entry.name_characters);
                lfn_vec[seq * 13 + 5..seq * 13 + 11].copy_from_slice(&entry.name_characters_second);
                lfn_vec[seq * 13 + 11..seq * 13 + 13].copy_from_slice(&entry.name_characters_third);
            } else {
                let entry = unsafe { entry.regular };
                let name = if !has_lfn {
                    let mut name = entry.name.clone();
                    let name = str::from_utf8(&name).ok()?.trim_right();
                    let extension = str::from_utf8(&entry.extension).ok()?.trim_right();

                    let mut name_str = String::from(name);
                    if extension.len() > 0 {
                        name_str.push_str(&".");
                        name_str.push_str(&extension);
                    }
                    name_str
                } else {
                    let len = lfn_vec.iter().position(|&c| c == 0x0000 || c == 0xFFFF)
                        .unwrap_or_else(|| lfn_vec.len());
                    String::from_utf16(&lfn_vec[..len]).ok()?
                };

                let first_cluster = Cluster::from((entry.first_cluster_number_high as u32) << 16
                    | entry.first_cluster_number_low as u32);

//                println!("name {}", &name);
                return Some(if entry.attributes.directory() {
                    Entry::Dir(Dir {
                        name,
                        first_cluster,
                        vfat: self.vfat.clone(),
                        metadata: entry.metadata(),
                    })
                } else {
                    Entry::File(File::new(name, first_cluster, self.vfat.clone(), entry.metadata(), entry.size_in_bytes))
                });
            }
        }
        None
    }
}


// FIXME: Implement `trait::Dir` for `Dir`.
impl traits::Dir for Dir {
    type Entry = Entry;
    type Iter = VFatIterator;

    fn entries(&self) -> io::Result<Self::Iter> {
        let mut buf: Vec<u8> = Vec::new();
        self.vfat.borrow_mut().read_chain(self.first_cluster, &mut buf)?;

        Ok(VFatIterator {
            entries: unsafe { buf.cast() }.into_iter(),
            vfat: self.vfat.clone(),
        })
    }
}
