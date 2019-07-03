use std::cmp::min;
use std::io::{self, SeekFrom};

use traits;
use vfat::{VFat, Shared, Cluster, Metadata};

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub first_cluster: Cluster,
    pub vfat: Shared<VFat>,
    pub metadata: Metadata,
    pub size: u32,
    ptr: u32,
}

impl File {
    pub fn new(name: String,
               first_cluster: Cluster,
               vfat: Shared<VFat>,
               metadata: Metadata,
               size: u32) -> File {
        File {
            name,
            first_cluster,
            vfat,
            metadata,
            size,
            ptr: 0,
        }
    }
}

// FIXME: Implement `traits::File` (and its supertraits) for `File`.

impl io::Read for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let can_read = min(buf.len(), (self.size as usize - self.ptr as usize));
        let mut data = Vec::new();
        let _read = self.vfat.borrow_mut().read_chain(self.first_cluster, &mut data)?;
        buf[..can_read as usize].copy_from_slice(&data[self.ptr as usize..(self.ptr as usize + can_read as usize)]);
        self.ptr += can_read as u32;
        Ok(can_read)
    }
}


impl io::Seek for File {
    /// Seek to offset `pos` in the file.
    ///
    /// A seek to the end of the file is allowed. A seek _beyond_ the end of the
    /// file returns an `InvalidInput` error.
    ///
    /// If the seek operation completes successfully, this method returns the
    /// new position from the start of the stream. That position can be used
    /// later with SeekFrom::Start.
    ///
    /// # Errors
    ///
    /// Seeking before the start of a file or beyond the end of the file results
    /// in an `InvalidInput` error.
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match pos {
            SeekFrom::Start(offset) => {
                if offset as u32 > self.size {
                    Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid Position"))
                } else {
                    self.ptr = offset as u32;
                    Ok(self.ptr as u64)
                }
            }
            SeekFrom::End(offset) => {
                if offset as i64 > 0 || (self.size as i64 + offset as i64) < 0 {
                    Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid Position"))
                } else {
                    self.ptr = (self.size as i64 + offset as i64) as u32;
                    Ok(self.ptr as u64)
                }
            }
            SeekFrom::Current(offset) => {
                let new_ptr: i64 = self.ptr as i64 + offset as i64;
                if new_ptr >= self.size as i64 || new_ptr < 0 {
                    Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid Position"))
                } else {
                    self.ptr = new_ptr as u32;
                    Ok(new_ptr as u64)
                }
            }
        }
    }
}

impl io::Write for File {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        // Skip this as this is a read only filesystem
        Ok(0)
    }

    fn flush(&mut self) -> io::Result<()> {
        // Skip this as this is a read only filesystem
        Ok(())
    }
}

impl traits::File for File {
    fn sync(&mut self) -> io::Result<()> {
        // Skip this as this is a read only filesystem
        Ok(())
    }

    fn size(&self) -> u64 {
        self.size as u64
    }
}
