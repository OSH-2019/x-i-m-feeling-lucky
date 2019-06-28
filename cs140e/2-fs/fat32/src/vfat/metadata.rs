use std::fmt;

use traits;

/// A date as represented in FAT32 on-disk structures.
#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Date(u16);

impl Data {
    pub fn year(&self) -> usize {
        // bits 15-9 is year (minus 1980)
        ((self.0 >> 9) as usize & 0b1111111) + 1980
    }
    pub fn month(&self) -> u8 {
        // bits 10-5 is month
        (self.0 >> 5) as u8 & 0b1111
    }
    pub fn day(&self) -> u8 {
        // bits 4-0 is day
        self.0 as u8 & 0b11111
    }
}


/// Time as represented in FAT32 on-disk structures.
#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Time(u16);

impl Time {
    pub fn hour(&self) -> u8 {
        // bits 15-9 is year (minus 1980)
        (self.0 >> 11) as u8 & 0b11111
    }
    pub fn minute(&self) -> u8 {
        // bits 10-5 is month
        (self.0 >> 5) as u8 & 0b111111
    }
    pub fn second(&self) -> u8 {
        // bits 4-0 is day
        ((self.0 as u8) & 0b11111) * 2
    }
}


/// File attributes as represented in FAT32 on-disk structures.
#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Attributes(u8);

impl Attributes {
    //    READ_ONLY=0x01 HIDDEN=0x02 SYSTEM=0x04 VOLUME_ID=0x08
//    DIRECTORY=0x10 ARCHIVE=0x20
//    LFN=READ_ONLY|HIDDEN|SYSTEM|VOLUME_ID
    const READ_ONLY: u8 = 0x01;
    const HIDDEN: u8 = 0x02;
    const SYSTEM: u8 = 0x04;
    const VOLUME_ID: u8 = 0x08;
    const DIRECTORY: u8 = 0x10;
    const ARCHIVE: u8 = 0x20;
    const LFN: u8 = READ_ONLY | HIDDEN | SYSTEM | VOLUME_ID;

    pub fn read_only(&self) -> bool {
        self.0 == Self::READ_ONLY
    }
    pub fn hidden(&self) -> bool {
        self.0 == Self::HIDDEN
    }
    pub fn system(&self) -> bool {
        self.0 == Self::SYSTEM
    }
    pub fn volume_id(&self) -> bool {
        self.0 == Self::VOLUME_ID
    }
    pub fn directory(&self) -> bool {
        self.0 == Self::DIRECTORY
    }
    pub fn archive(&self) -> bool {
        self.0 == Self::ARCHIVE
    }
    pub fn lfn(&self) -> bool {
        self.0 == Self::LFN
    }
}

/// A structure containing a date and time.
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Timestamp {
    pub date: Date,
    pub time: Time,
}

/// Metadata for a directory entry.
#[derive(Default, Debug, Clone)]
pub struct Metadata {
    // FIXME: Fill me in.
    attributes: Attributes,
    create: Timestamp,
    access: Timestamp,
    modify: Timestamp,
}

// FIXME: Implement `traits::Timestamp` for `Timestamp`.
impl traits::Timestamp for Timestamp {
    fn year(&self) -> usize {
        self.date.year()
    }
    fn month(&self) -> usize {
        self.date.month()
    }
    fn day(&self) -> usize {
        self.date.day()
    }
    fn hour(&self) -> usize {
        self.time.hour()
    }
    fn minute(&self) -> usize {
        self.time.minute()
    }
    fn second(&self) -> usize {
        self.time.second()
    }
}

// FIXME: Implement `traits::Metadata` for `Metadata`.
impl traits::Metadata for Metadata {
    fn read_only(&self) -> bool {
        self.attributes.read_only()
    }
    fn hidden(&self) -> bool {
        self.attributes.hidden()
    }
    fn system(&self) -> bool {
        self.attributes.system()
    }
    fn volume_id(&self) -> bool {
        self.attributes.volume_id()
    }
    fn directory(&self) -> bool {
        self.attributes.directory()
    }
    fn archive(&self) -> bool {
        self.attributes.archive()
    }
    fn lfn(&self) -> bool {
        self.attributes.lfn()
    }
}

// FIXME: Implement `fmt::Display` (to your liking) for `Metadata`.


// Not do these, instead use std::fmt::DebugStruct
//impl fmt::Display for Attributes {}
//impl fmt::Display for Timestamp {}

// use std::fmt::DebugStruct
impl fmt::Display for Metadata {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Metadata")
            .field("attributes", &self.attributes)
            .field("create", &self.create)
            .field("access", &self.access)
            .field("modify", &self.modify)
            .finish()
    }

}
