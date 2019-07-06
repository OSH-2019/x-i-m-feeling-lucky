use traps::TrapFrame;
use SCHEDULER;
use pi::timer::current_time;
use process::{State, Process};

/// Sleep for `ms` milliseconds.
///
/// This system call takes one parameter: the number of milliseconds to sleep.
///
/// In addition to the usual status value, this system call returns one
/// parameter: the approximate true elapsed time from when `sleep` was called to
/// when `sleep` returned.
pub fn sleep(ms: u32, tf: &mut TrapFrame) {
    let start = current_time();
    let end = start + (ms as u64) * 1000;

    let sleeping = Box::new(move |process: &mut Process| {
        let current = current_time();
        if current > end {
            // x7 = 0; succeed
            process.trap_frame.x1_x29[6] = 0;
            // return x0 = elapsed time in ms
            process.trap_frame.x0 = (current - start) / 1000;
            true
        } else {
            false
        }
    });
    SCHEDULER.switch(State::Waiting(sleeping), tf).unwrap();
}


//according to the svc num to handle system call
pub fn handle_syscall(num: u16, tf: &mut TrapFrame) {
    match num {
        //sleep syscall
        1 => {
            sleep(tf.x0 as u32, tf);
        }
        // kill syscall
        2 => {
            tf.x1_x29[6] = match SCHEDULER.remove(tf.x0, tf) {
                Some(_) => 0,
                None => 1,
            };
        }
        //currently unexist, set x7 to 1
        _ => {
            tf.x1_x29[6] = 1;
        }
    }
}
