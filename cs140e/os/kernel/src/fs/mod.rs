pub mod sd;

use std::io;
use std::path::{Path, PathBuf};

use fat32::vfat::{self, Shared, VFat};
pub use fat32::traits;

use mutex::Mutex;
use self::sd::Sd;

pub struct FileSystem(Mutex<Option<Shared<VFat>>>);

impl FileSystem {
    /// Returns an uninitialized `FileSystem`.
    ///
    /// The file system must be initialized by calling `initialize()` before the
    /// first memory allocation. Failure to do will result in panics.
    pub const fn uninitialized() -> Self {  // add const
        FileSystem(Mutex::new(None))
    }

    /// Initializes the file system.
    ///
    /// # Panics
    ///
    /// Panics if the underlying disk or file sytem failed to initialize.
    pub fn initialize(&self) {
        //unimplemented!("FileSystem::initialize()")
        *self.0.lock() = Some(VFat::from(Sd::new().unwrap()).unwrap());
    }

    fn get_vfat(&self) -> io::Result<Shared<VFat>> {
        match *self.0.lock() {
            Some(ref vfat) => Ok(vfat.clone()),
            None => Err(io::Error::new(io::ErrorKind::NotConnected,
                                       "Not initialized")),
        }
    }
}

// FIXME: Implement `fat32::traits::FileSystem` for a useful type.
impl<'a> traits::FileSystem for &'a FileSystem {
    type File = vfat::File;
    type Dir = vfat::Dir;
    type Entry = vfat::Entry;

    fn open<P: AsRef<Path>>(self, path: P) -> io::Result<Self::Entry> {
        self.get_vfat()?.open(path)
    }

    fn create_file<P: AsRef<Path>>(self, path: P) -> io::Result<Self::File> {
        self.get_vfat()?.create_file(path)
    }

    fn create_dir<P>(self, path: P, parents: bool) -> io::Result<Self::Dir>
        where P: AsRef<Path>
    {
        self.get_vfat()?.create_dir(path, parents)
    }

    fn rename<P, Q>(self, from: P, to: Q) -> io::Result<()>
        where P: AsRef<Path>, Q: AsRef<Path>
    {
        self.get_vfat()?.rename(from, to)
    }

    fn remove<P: AsRef<Path>>(self, path: P, children: bool) -> io::Result<()> {
        self.get_vfat()?.remove(path, children)
    }

    fn canonicalize<P: AsRef<Path>>(self, path: P) -> io::Result<PathBuf> {
        self.get_vfat()?.canonicalize(path)
    }
}
