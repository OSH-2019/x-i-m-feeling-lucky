use std::fmt;
use alloc::heap::{AllocErr, Layout};
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
        let mut list = [LinkedList::new(); BIN_SIZE];
		
		//construct the free-space list.
		while current + MIN_SIZE <= end {
			//decide the size of chunk.
			let size = min((end - current).next_power_of_two() >> 1, 
							1 << current.trailing_zeros());
			
			//push the chunk to list and renew the members
			unsafe {
				list[size.trailing_zeros() as usize].push(current as *mut usize);
			}
			current += size;
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
		if !layout.align().is_power_of_two() || layout.size() <= 0 {
			return Err(AllocErr::Unsupported{ details: "Unsupported"});
		}
		
		//decide allocating size
        let size = max(layout.size().next_power_of_two(),
						max(layout.align(), MIN_SIZE));
		let index = size.trailing_zeros() as usize;
		
		//allocate chunk and adjust the list
		for i in index..self.list.len() {
			if self.list[i].is_empty() { continue }
			//handle the fragments， using buddy system like linux
			for j in (index + 1..i + 1).rev() {
				let block = self.list[j].pop().expect("bigger block should have free space");
				unsafe {
					self.list[j-1].push((block as usize + (1 << (j - 1))) as *mut usize);
                	self.list[j-1].push(block);
				}
			}
			//return the start address
			let start = Ok(self.list[index].pop().expect("current block should have free space now") as *mut u8);
			self.allocated += size;
			return start;
		}
		
		//cannot find proper chunk, the memory is exhausted
		return Err(AllocErr::Exhausted{ request: layout});
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
		let index = size.trailing_zeros() as usize; 

		//deallocate the chunk and trying to merge chunks according to buddy system strategy
		unsafe{
			self.list[index].push(ptr as *mut usize);
			let mut addr = ptr as usize;
			let mut current = index;
			loop {
				let mut flag = false;
				let buddy = addr ^ (1 << current);
				//try to find the buddy chunk
				for node in self.list[current].iter_mut() {
					if node.value() as usize == buddy {
						//find the buddy chunk, set flag to true
						node.pop();
						flag = true;
						break;
					}
				}
			
				//according to the flag 
				if flag {
					//find the buddy, go to the next list
					self.list[current].pop();
                	addr = min(addr, buddy);
					current += 1;
                	self.list[current].push(addr as *mut usize);
				} else {
					//cannot find the buddy, break
					break;
				}
			}
		}

		self.allocated -= size;
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
