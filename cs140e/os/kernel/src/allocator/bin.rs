use std::fmt;
use alloc::alloc::{AllocErr, Layout};
use std::cmp::{min, max};
use std::mem::size_of;

use allocator::util::*;
use allocator::linked_list::LinkedList;

/// Any allocation or deallocation request for less than or equal to 2^3 bytes would be handled by the 2^3 bin
/// And request between 2^3 to 2^4 bytes from the 2^4 bin, and so on. 
///	Request cannot be over 2^32 = 4GB
const BIN_SIZE: usize = 32;
const MIN_BITS: usize = 3;
const MIN_SIZE: usize = 8;

/// A simple allocator that allocates based on size classes.
pub struct Allocator {
    // FIXME: Add the necessary fields.
    list: [LinkedList; BIN_SIZE],
    allocated: usize,
    total: usize,
}

impl Allocator {
    /// Creates a new bin allocator that will allocate memory from the region
    /// starting at address `start` and ending at address `end`.
    pub fn new(start: usize, end: usize) -> Allocator {
		//initialize the members.
        let mut current = start;
		let mut total = 0;
        let mut list = [LinkedList::new(),BIN_SIZE];
		
		//construct the free-space list.
		while current + MIN_SIZE <= end {
			//decide the size of chunk.
			let size = min((end - start).next_power_of_two() >> 2, 
							1 << start.trailing_zeros());
			
			//push the chunk to list and renew the members
			unsafe {
				list[size.trailing_zeros() as usize].push(start as *mut usize);
			}
			start += size;
			total += size;
		}
		
		//return the allocator
		Allocator{
			list: list,
			allocated: 0,
			total: total,
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
		//error demand handling
		if !layout.align.is_power_of_two || layout.size() <= 0 {
			return Err(AllocErr::Unsupported {
				details: "unsupported demand"});
		}
		
		//decide allocating size
        let size = max(layout.size().next_power_of_two(),
						max(layout.align(), MIN_SIZE));
		let index = size.trailing_zeros();
		
		//allocate chunk and adjust the list
		for i in index..self.list.len() {
			if self.list[i].is_empty() { continue }
			let start = self.list[i].pop().unwrap();
			//handle the fragments， using buddy system like linux
			for j in index..i {
				unsafe {
					let buddy = start.add(1 << j) as *mut usize;
					self.list[j].push(buddy);
				}
			}
			//return the start address
			return Ok(start);
		}
		
		//cannot find proper chunk, the memory is exhausted
		return Err(AllocErr::Exhausted{
			details: "memory runs out"});
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
    pub fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
		//decide deallocating size
        let size = max(layout.size().next_power_of_two(),
						max(layout.align(), MIN_SIZE));
		let index = size.trailing_zeros(); 

		//deallocate the chunk and trying to merge chunks according to buddy system strategy
		let mut addr = ptr as usize;
		for i in index..self.list.len() {
			let mut flag = false;
			let buddy = addr ^ (1 << i);
			
			//try to find the buddy chunk
			for node in self.list[i].iter_mut() {
				if node.value() as usize == addr {
					//find the buddy chunk, go to the next list
					node.pop();
					flag = true;
					addr = min(addr, buddy);
					break;
				}
			}
			
			//cannot find the buddy, just push that chunk
			if !flag {
				unsafe {
					self.list[i].push(addr as *mut usize);
				}
			}
		}
    }
}
//
// FIXME: Implement `Debug` for `Allocator`.
impl fmt::Debug for Allocator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Bin Allocator")
			.field("allocated", &self.allocated)
			.field("total", &self.total)
			.field("list", &self.list)
			.finish()
    }
}