use alloc::heap::{AllocErr, Layout};

use allocator::util::*;

/// A "bump" allocator: allocates memory by bumping a pointer; never frees.
#[derive(Debug)]
pub struct Allocator {
    current: usize,
    end: usize,
}

impl Allocator {
    /// Creates a new bump allocator that will allocate memory from the region
    /// starting at address `start` and ending at address `end`.
    pub fn new(start: usize, end: usize) -> Allocator {
        //unimplemented!("bump allocator")
        Allocator{
            current:start,
            end:end,
        }
    }

    /// Allocates memory. Returns a pointer meeting the size and alignment
    /// properties of `layout.size()` and `layout.align()`.
    ///
    /// If this method returns an `Ok(addr)`, `addr` will be non-null address
    /// pointing to a block of storage suitable for holding an instance of
    /// `layout`. In particular, the block will be at least `layout.size()`
    /// bytes large and will be aligned to `layout.align()`. The returned block
    /// of storage may or may not have its contents initialized or zeroed.
    ///
    /// # Safety
    ///
    /// The _caller_ must ensure that `layout.size() > 0` and that
    /// `layout.align()` is a power of two. Parameters not meeting these
    /// conditions may result in undefined behavior.
    ///
    /// # Errors
    ///
    /// Returning `Err` indicates that either memory is exhausted
    /// (`AllocError::Exhausted`) or `layout` does not meet this allocator's
    /// size or alignment constraints (`AllocError::Unsupported`).
    pub fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        //unimplemented!("bump allocation")
        if layout.size() <= 0 || !layout.align().is_power_of_two() {return Err(AllocErr::Unsupported{ details: "Unsupported"});}

        let newaddr = align_up(self.current,layout.align());
        let newcurrent = newaddr.saturating_add(layout.size());
        if newcurrent < self.end && newcurrent > newaddr {
            self.current = newcurrent;
            Ok(newaddr as *mut u8)
        }
        else{
            Err(AllocErr::Exhausted{request: layout})
        }
    }

    /// Deallocates the memory referenced by `ptr`.
    ///
    /// # Safety
    ///
    /// The _caller_ must ensure the following:
    ///
    ///   * `ptr` must denote a block of memory currently allocated via this
    ///     allocator
    ///   * `layout` must properly represent the original layout used in the
    ///     allocation call that returned `ptr`
    ///
    /// Parameters not meeting these conditions may result in undefined
    /// behavior.
    pub fn dealloc(&mut self, _ptr: *mut u8, _layout: Layout) {
        //unimplemented!("bump deallocation")
        //nothing
    }
}
