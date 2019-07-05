#![feature(lang_items)]
#![feature(inclusive_range_syntax)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(exclusive_range_pattern)]
#![feature(i128_type)]
#![feature(never_type)]
#![feature(unique)]
#![feature(pointer_methods)]
#![feature(naked_functions)]
#![feature(fn_must_use)]
#![feature(alloc, allocator_api, global_allocator)]

#[macro_use]
#[allow(unused_imports)]
extern crate alloc;
extern crate pi;
extern crate stack_vec;
extern crate fat32;

pub mod allocator;
pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;
pub mod fs;
pub mod traps;
pub mod aarch64;
pub mod process;
pub mod vm;

#[cfg(not(test))]
use allocator::Allocator;
use fs::FileSystem;
use process::GlobalScheduler;
use process::sys_sleep;
use pi::gpio::Gpio;
use pi::timer::spin_sleep_ms;

#[cfg(not(test))]
#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();

pub static FILE_SYSTEM: FileSystem = FileSystem::uninitialized();

pub static SCHEDULER: GlobalScheduler = GlobalScheduler::uninitialized();

#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn kmain() {
    ALLOCATOR.initialize();
    //#[cfg(feature = "qemu")]
    //Timer::initialize();
    FILE_SYSTEM.initialize();
    spin_sleep_ms(2000);
    SCHEDULER.start();
}

pub extern fn run_shell() {
    loop {
        shell::shell("$ ");
    }
}

pub extern fn run_blinky() {
    let mut ready_led = Gpio::new(16).into_output();
    loop {
        ready_led.set();
        sys_sleep(500);
        ready_led.clear();
        sys_sleep(500);
    }
}
