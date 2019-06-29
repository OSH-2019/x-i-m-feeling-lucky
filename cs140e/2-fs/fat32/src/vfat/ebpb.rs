use std::{fmt,mem};

use traits::BlockDevice;
use vfat::Error;

const EBPB_SIZE: usize = 512;

#[repr(C, packed)]
pub struct BiosParameterBlock {
    disassemble_to_jump: [u8; 3],
    oem_identifier: [u8; 8],
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sectors: u16,
    num_of_fat: u8,
    max_of_dir_entries: u16,
    total_logical_sectors: u16,
    media_descriptor_type: u8,
    sectors_per_fat: u16,
    sectors_per_track: u16,
    heads_or_sides:  u16,
    hidden_sectors: u32,
    total_logical_sectors_assist: u32,

    // EBPB
    sectors_per_fat_assist: u32,
    flags: u16,
    fat_version_number: u16,
    root_dir_cluster_number: u32,
    fsinfo_sector_number: u16,
    backup_boot_sector_number: u16,
    driver_number: u8,
    flags_in_windows_nt: u8,
    signature: u8,
    volume_id_serial_number: u32,
    volume_label_string: [u8; 11],
    system_identifier_string: [u8; 8],
    boot_code: [u8; 420],
    bootable_signature: u16
}

impl BiosParameterBlock {
    /// Reads the FAT32 extended BIOS parameter block from sector `sector` of
    /// device `device`.
    ///
    /// # Errors
    ///
    /// If the EBPB signature is invalid, returns an error of `BadSignature`.
    pub fn from<T: BlockDevice>(
        mut device: T,
        sector: u64
    ) -> Result<BiosParameterBlock, Error> {
        let mut buf = [0u8; EBPB_SIZE];
        let bytes_read = device.read_sector(0, &buf);

        let ebpb: BiosParameterBlock = unsafe { mem::transmute(buf)};
        if ebpb.signature != [0xAA, 0x55] {
            return Err(Error::BadSignature);
        }

        Ok(ebpb)
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
            .field("boot_code", &self.boot_code)
            .field("bootable_signature", &self.bootable_signature)
            .finish()
    }
}
