use common::IO_BASE;
use volatile::prelude::*;
use volatile::{Volatile, ReadVolatile};

/// The base address for the ARM system timer registers.
const TIMER_REG_BASE: usize = IO_BASE + 0x3000;

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    CS: Volatile<u32>,
    CLO: ReadVolatile<u32>,
    CHI: ReadVolatile<u32>,
    COMPARE: [Volatile<u32>; 4]
}

/// The Raspberry Pi ARM system timer.
pub struct Timer {
    registers: &'static mut Registers
}

impl Timer {
    /// Returns a new instance of `Timer`.
    pub fn new() -> Timer {
        Timer {
            registers: unsafe { &mut *(TIMER_REG_BASE as *mut Registers) },
        }
    }

    /// Reads the system timer's counter and returns the 64-bit counter value.
    /// The returned value is the number of elapsed microseconds.
    pub fn read(&self) -> u64 {
        let lo = self.registers.CLO.read();
        let hi = self.registers.CHI.read();
        ((hi as u64) << 32) | (lo as u64)
    }

    /// Sets up a match in timer 1 to occur `us` microseconds from now. If
    /// interrupts for timer 1 are enabled and IRQs are unmasked, then a timer
    /// interrupt will be issued in `us` microseconds.
    pub fn tick_in(&mut self, us: u32) {
        let target = self.registers.CLO.read().wrapping_add(us); //compute the time
        self.registers.COMPARE[1].write(target);  // set up a match in timer 1
        self.registers.CS.or_mask(0b0010);        // clear timer 1 interrupt
    }
}

/// Returns the current time in microseconds.
pub fn current_time() -> u64 {
    //let timer = Timer::new();
    //timer.read()
    Timer::new().read()
}

/// Spins until `us` microseconds have passed.
pub fn spin_sleep_us(us: u64) {
    let start = current_time();
    let end = start + us;
    loop{
        if current_time() >= end {
            break;
        }
    }
}

/// Spins until `ms` milliseconds have passed.
pub fn spin_sleep_ms(ms: u64) {
    spin_sleep_us(ms*1000);
}

/// Sets up a match in timer 1 to occur `us` microseconds from now. If
/// interrupts for timer 1 are enabled and IRQs are unmasked, then a timer
/// interrupt will be issued in `us` microseconds.
pub fn tick_in(us: u32) {
    Timer::new().tick_in(us);
}
