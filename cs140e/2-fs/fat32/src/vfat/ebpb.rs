use std::{fmt, mem, io};

use traits::BlockDevice;
use vfat::Error;

#[repr(C, packed)]
pub struct BiosParameterBlock {
    pub disassemble_to_jump: [u8; 3],
    pub oem_identifier: [u8; 8],
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors: u16,
    pub num_of_fat: u8,
    pub max_of_dir_entries: u16,
    pub total_logical_sectors: u16,
    pub media_descriptor_type: u8,
    pub sectors_per_fat: u16,
    pub sectors_per_track: u16,
    pub heads_or_sides: u16,
    pub hidden_sectors: u32,
    pub total_logical_sectors_assist: u32,

    // EBPB
    pub sectors_per_fat_assist: u32,
    pub flags: u16,
    pub fat_version_number: u16,
    pub root_dir_cluster_number: u32,
    pub fsinfo_sector_number: u16,
    pub backup_boot_sector_number: u16,
    reserved_and_should_be_zero: [u8; 12],
    pub driver_number: u8,
    pub flags_in_windows_nt: u8,
    pub signature: u8,
    pub volume_id_serial_number: u32,
    pub volume_label_string: [u8; 11],
    pub system_identifier_string: [u8; 8],
    pub boot_code: [u8; 420],
    pub bootable_signature: u16,
}

impl BiosParameterBlock {
    /// Reads the FAT32 extended BIOS parameter block from sector `sector` of
    /// device `device`.
    ///
    /// # Errors
    ///
    /// If the EBPB signature is invalid, returns an error of `BadSignature`.
    pub fn from<T: BlockDevice>(mut device: T, sector: u64)
                                -> Result<BiosParameterBlock, Error> {
        let mut buf = [0u8; mem::size_of::<BiosParameterBlock>()];
        let bytes_read = device.read_sector(sector, &mut buf);

        let ebpb: BiosParameterBlock = unsafe { mem::transmute(buf) };
//        let ebpb = Self::modify_byte_order(ebpb);

        if ebpb.bootable_signature != 0xAA55 {
            return Err(Error::BadSignature);
        }

        Ok(ebpb)
    }

    pub fn sectors_per_fat(&self) -> u32 {
        if self.sectors_per_fat != 0 {
            self.sectors_per_fat as u32
        } else {
            self.sectors_per_fat_assist
        }
    }
}


impl fmt::Debug for BiosParameterBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BiosParameterBlock")
            .field("disassemble_to_jump", &self.disassemble_to_jump)
            .field("oem_identifier", &self.oem_identifier)
            .field("bytes_per_sector", &self.bytes_per_sector)
            .field("sectors_per_cluster", &self.sectors_per_cluster)
            .field("reserved_sectors", &self.reserved_sectors)
            .field("num_of_fat", &self.num_of_fat)
            .field("max_of_dir_entries", &self.max_of_dir_entries)
            .field("total_logical_sectors", &self.total_logical_sectors)
            .field("media_descriptor_type", &self.media_descriptor_type)
            .field("sectors_per_fat", &self.sectors_per_fat)
            .field("sectors_per_track", &self.sectors_per_track)
            .field("heads_or_sides", &self.heads_or_sides)
            .field("hidden_sectors", &self.hidden_sectors)
            .field("total_logical_sectors_assist", &self.total_logical_sectors_assist)

            .field("sectors_per_fat_assist", &self.sectors_per_fat_assist)
            .field("flags", &self.flags)
            .field("fat_version_number", &self.fat_version_number)
            .field("root_dir_cluster_number", &self.root_dir_cluster_number)
            .field("fsinfo_sector_number", &self.fsinfo_sector_number)
            .field("backup_boot_sector_number", &self.backup_boot_sector_number)
            .field("driver_number", &self.driver_number)
            .field("flags_in_windows_nt", &self.flags_in_windows_nt)
            .field("signature", &self.signature)
            .field("volume_id_serial_number", &self.volume_id_serial_number)
            .field("volume_label_string", &self.volume_label_string)
            .field("system_identifier_string", &self.system_identifier_string)
//            .field("boot_code", &self.boot_code)
            .field("bootable_signature", &self.bootable_signature)
            .finish()
    }
}
