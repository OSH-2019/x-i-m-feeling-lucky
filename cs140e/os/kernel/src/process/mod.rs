mod process;
mod state;
mod scheduler;
mod stack;

pub use self::process::{Process, Id};
pub use self::state::State;
pub use self::scheduler::{GlobalScheduler, TICK};
pub use self::stack::Stack;

//method for implement sleep(syscall 1)
pub fn sys_sleep(ms: u32) -> u32 {
    let error: u64;
    let result: u32;
    unsafe {
        asm!("mov x0, $2
              svc 1
              mov $0, x0
              mov $1, x7"
              : "=r"(result), "=r"(error)
              : "r"(ms)
              : "x0", "x7")
    }
    //if executed succesfully, the content of x7 is 0
    assert_eq!(error, 0);
    result
}

pub fn sys_kill(kill_id: u32) -> bool {
    let error: u64;
    unsafe {
        asm!("mov x0, $1
              svc 2
              mov $0, x7"
              : "=r"(error)
              : "r"(kill_id)
              : "x7")
    }
    //if executed succesfully, the content of x7 is 0
    match error {
        0 => true,
        _ => false,
    }
}

pub fn sys_process_terminated(){
    //let terminated_id: u64;
    unsafe {
        asm!("svc 3
              ")
    }
    //if executed succesfully, the content of x7 is 0
    //terminated_id
}
