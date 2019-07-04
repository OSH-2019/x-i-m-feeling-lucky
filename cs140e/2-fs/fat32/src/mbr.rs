use std::{mem, fmt, io};

use traits::BlockDevice;

const MBR_SIZE: usize = 512;

#[derive(Debug)]
pub enum Error {
    /// There was an I/O error while reading the MBR.
    Io(io::Error),
    /// Partition `.0` (0-indexed) contains an invalid or unknown boot indicator.
    UnknownBootIndicator(u8),
    /// The MBR magic signature was invalid.
    BadSignature,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct CHS {
    pub head: u8,
    pub sector_and_cylinder: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PartitionType(u8);

impl PartitionType {
    pub fn is_fat(&self) -> bool {
        self.0 == 0x0B || self.0 == 0x0C
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct PartitionEntry {
    pub indicator: u8,
    pub starting_chs: CHS,
    pub partition_type: PartitionType,
    pub ending_chs: CHS,
    pub relative_sector: u32,
    pub total_sectors: u32,
}

impl PartitionEntry {
    pub fn valid_indicator(&self) -> bool {
        self.indicator == 0x00 || self.indicator == 0x80
    }
}

/// The master boot record (MBR).
#[repr(C, packed)]
pub struct MasterBootRecord {
    pub bootstrap: [u8; 436],
    pub disk_id: [u8; 10],
    pub partition_entries: [PartitionEntry; 4],
    pub signature: [u8; 2],
}

impl MasterBootRecord {
    /// Reads and returns the master boot record (MBR) from `device`.
    ///
    /// # Errors
    ///
    /// Returns `BadSignature` if the MBR contains an invalid magic signature.
    /// Returns `UnknownBootIndicator(n)` if partition `n` contains an invalid
    /// boot indicator. Returns `Io(err)` if the I/O error `err` occured while
    /// reading the MBR.
    pub fn from<T: BlockDevice>(mut device: T) -> Result<MasterBootRecord, Error> {
        let mut buf = [0u8; MBR_SIZE];
        let _ = device.read_sector(0, &mut buf);

        let mbr: MasterBootRecord = unsafe { mem::transmute(buf) };

        if !mbr.valid_signature() {
            return Err(Error::BadSignature);
        }

        for i in 0..4 {
            if !mbr.partition_entries[i].valid_indicator() {
                return Err(Error::UnknownBootIndicator(i as u8));
            }
        }

        Ok(mbr)
    }


    pub fn first_fat32(&self) -> Option<&PartitionEntry> {
        self.partition_entries.iter()
            .find(|part| part.partition_type.is_fat())
    }

    fn valid_signature(&self) -> bool {
        self.signature == [0x55, 0xAA]
    }
}

impl fmt::Debug for MasterBootRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MasterBootRecord")
            .field("bootstrap", &"Not available for print.")
            .field("disk_id", &self.disk_id)
            .field("partition_entry_0", &self.partition_entries[0])
            .field("partition_entry_1", &self.partition_entries[1])
            .field("partition_entry_2", &self.partition_entries[2])
            .field("partition_entry_3", &self.partition_entries[3])
            .finish()
    }
}