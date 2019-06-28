use std::ffi::OsStr;
use std::char::decode_utf16;
use std::borrow::Cow;
use std::io;

use traits;
use util::VecExt;
use vfat::{VFat, Shared, File, Cluster, Entry};
use vfat::{Metadata, Attributes, Timestamp, Time, Date};

#[derive(Debug)]
pub struct Dir {
    pub name: String,
    pub lfn: String,
    pub first_cluster: Cluster,
    pub vfat: Shared<VFat>,
    pub metadata: Metadata,

}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct VFatRegularDirEntry {
    name: [u8;8],
    extension: [u8;3],
    attributes: Attributes,
    reserved_by_windows_nt: u8,
    creation_time_in_tenths: u8,
    create_time: Time,
    create_date: Date,
    access_time: Time,
    access_data: Data,
    first_cluster_number_high: u16,
    modify_time: Time,
    modify_data: Data,
    first_cluster_number_low: u16,
    size_in_bytes: u32,

}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct VFatLfnDirEntry {
    sequence_number: u8,
    name_characters: [u8;10],
    attributes: u8,
    lfn_type: u8,
    checksum: u8,
    name_characters_second: [u8;12],
    always_zero: u16,
    name_characters_third: [u8;4],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct VFatUnknownDirEntry {
    _placeholder0: [u8;11],
    attributes: u8,
    _placeholder1: [u8;20],
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
        unimplemented!("Dir::find()")
    }
}

// FIXME: Implement `trait::Dir` for `Dir`.
impl traits::Dir for Dir {
    type Entry = Entry;
    type Iter = VFatEntryIterator;

    fn entries(&self) -> io::Result<Self::Iter> {
        let mut data: Vec<u8> = Vec::new();
        self.vfat.borrow_mut().read_chain(self.first_cluster, &mut data);
    }
}