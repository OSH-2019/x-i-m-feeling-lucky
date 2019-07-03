#![allow(dead_code)]
/// Align `addr` downwards to the nearest multiple of `align`.
///
/// The returned usize is always <= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
    if !align.is_power_of_two() {
        panic!("align_down: align is not power of 2");
    }
    addr - addr % align
}

/// Align `addr` upwards to the nearest multiple of `align`.
///
/// The returned `usize` is always >= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
pub fn align_up(addr: usize, align: usize) -> usize {
    if !align.is_power_of_two() {
        panic!("align_up: align is not power of 2");
    }
    if addr % align == 0 {
        addr
    } else {
        addr + (align - addr % align)
    }
}
