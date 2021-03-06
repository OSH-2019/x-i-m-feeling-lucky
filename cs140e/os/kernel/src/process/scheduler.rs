use std::collections::VecDeque;
use console::kprintln;
use mutex::Mutex;
use process::{Process, State, Id};
use traps::TrapFrame;
use pi::interrupt::{Controller, Interrupt};
use pi::timer::tick_in;
use pi::timer::spin_sleep_ms;
use aarch64;
use run_shell;
use run_blinky;
use run_blinky2;
use process::sys_process_terminated;

/// The `tick` time.
// FIXME: When you're ready, change this to something more reasonable.
pub const TICK: u32 = 100 * 1000; // us
//pub const TICK: u32 = ::std::u32::MAX;

/// Process scheduler for the entire machine.
#[derive(Debug)]
pub struct GlobalScheduler(Mutex<Option<Scheduler>>);

impl GlobalScheduler {
    /// Returns an uninitialized wrapper around a local scheduler.
    pub const fn uninitialized() -> GlobalScheduler {
        GlobalScheduler(Mutex::new(None))
    }

    /// Adds a process to the scheduler's queue and returns that process's ID.
    /// For more details, see the documentation on `Scheduler::add()`.
    pub fn add(&self, process: Process) -> Option<Id> {
        self.0.lock().as_mut().expect("scheduler uninitialized").add(process)
    }

    /// Performs a context switch using `tf` by setting the state of the current
    /// process to `new_state`, saving `tf` into the current process, and
    /// restoring the next process's trap frame into `tf`. For more details, see
    /// the documentation on `Scheduler::switch()`.
    #[must_use]
    pub fn switch(&self, new_state: State, tf: &mut TrapFrame) -> Option<Id> {
        self.0.lock().as_mut().expect("scheduler uninitialized").switch(new_state, tf)
    }

    pub fn remove(&self, remove_id: Id, tf: &mut TrapFrame) -> Option<Id> {
        self.0.lock().as_mut().expect("scheduler uninitialized").remove(remove_id,tf)
    }

    pub fn current(&self) -> Vec<Id> {
        self.0.lock().as_mut().expect("scheduler uninitialized").current()
    }

    /// Initializes the scheduler and starts executing processes in user space
    /// using timer interrupt based preemptive scheduling. This method should
    /// not return under normal conditions.
    pub fn start(&self) {
        //unimplemented!()
        spin_sleep_ms(2000);
        *self.0.lock() = Some(Scheduler::new());
        let mut process = Process::new().expect("First process failed");
        process.trap_frame.ELR = run_shell as u64;
        process.trap_frame.SP = process.stack.top().as_u64();
        // Don't mask DAIF, set execution level to 0.
        process.trap_frame.SPSR = 0b1101_00_0000;
        let tf = process.trap_frame.clone();
        self.add(process);

        let mut process_1 = Process::new().unwrap();
        process_1.trap_frame.ELR = run_blinky as u64;
        process_1.trap_frame.SP = process_1.stack.top().as_u64();
        process_1.trap_frame.SPSR = 0b1101_00_0000;
        self.add(process_1);

        let mut process_2 = Process::new().unwrap();
        process_2.trap_frame.ELR = run_blinky2 as u64;
        process_2.trap_frame.SP = process_2.stack.top().as_u64();
        process_2.trap_frame.SPSR = 0b1101_00_0000;
        process_2.trap_frame.x30 = sys_process_terminated as u64;
        self.add(process_2);

        Controller::new().enable(Interrupt::Timer1);
        tick_in(TICK);

//        kprintln!("Beginning");

        // Switch to process.
        unsafe {
            asm!(
                "// Set the SP to the start of the trap frame.
                 mov SP, $0
                 // Restore the trap frame registers.
                 bl context_restore
                 // Reset SP to _start.
                 adr x0, _start
                 mov SP, x0
                 mov x0, xzr
                 mov x30, xzr
                 // Start the process.
                 eret"
                 :: "r"(tf)
                 :: "volatile");
        }
        kprintln!("Exiting");
    }
}

#[derive(Debug)]
struct Scheduler {
    processes: VecDeque<Process>,
    current: Option<Id>,
    last_id: Option<Id>,
}

impl Scheduler {
    /// Returns a new `Scheduler` with an empty queue.
    fn new() -> Scheduler {
        let processes = VecDeque::new();
        Scheduler {
            processes,
            current: None,
            last_id: None,
        }
    }

    /// Adds a process to the scheduler's queue and returns that process's ID if
    /// a new process can be scheduled. The process ID is newly allocated for
    /// the process and saved in its `trap_frame`. If no further processes can
    /// be scheduled, returns `None`.
    ///
    /// If this is the first process added, it is marked as the current process.
    /// It is the caller's responsibility to ensure that the first time `switch`
    /// is called, that process is executing on the CPU.
    fn add(&mut self, mut process: Process) -> Option<Id> {
        //generate id(checked_add to handle overflow)
        let id = match self.last_id {
            Some(id) => id.checked_add(1)?,
            None => 0
        };
        //set id in trap_frame
        process.trap_frame.TPIDR = id;
        //add the process in the queue
        self.processes.push_back(process);
        if let None = self.current {
            self.current = Some(id);
        }
        self.last_id = Some(id);
        //return the id
        self.last_id
    }

    /// Sets the current process's state to `new_state`, finds the next process
    /// to switch to, and performs the context switch on `tf` by saving `tf`
    /// into the current process and restoring the next process's trap frame
    /// into `tf`. If there is no current process, returns `None`. Otherwise,
    /// returns `Some` of the process ID that was context switched into `tf`.
    ///
    /// This method blocks until there is a process to switch to, conserving
    /// energy as much as possible in the interim.
    fn switch(&mut self, new_state: State, tf: &mut TrapFrame) -> Option<Id> {
        //get the current process for scheduling
        let mut current_process = self.processes.pop_front()?;
        let current_id = current_process.get_id();

        current_process.trap_frame = Box::new(*tf);
        current_process.state = new_state;
        self.processes.push_back(current_process);

        //find the next process to switch to
        loop {
            let mut next_process = self.processes.pop_front()?;
            if next_process.is_ready() {
                *tf = *next_process.trap_frame;
                next_process.state = State::Running;
                self.processes.push_front(next_process);
                break;
            } else if next_process.get_id() == current_id {
                //block until there is a process to switch to
                aarch64::wfi();
            }
            self.processes.push_back(next_process);
        }
        //return the current id is ok
        self.current
    }

    fn remove(&mut self, remove_id: Id, tf: &mut TrapFrame) -> Option<Id> {
        if self.processes.get(0).unwrap().get_id() == remove_id {
            self.switch(State::Ready, tf);
            self.processes.pop_back();
            Some(remove_id)
        }
        else {
            let mut rmindex = None;
            for (index,process) in self.processes.iter().enumerate() {
                if process.get_id() == remove_id {
                    //self.processes.remove(index);
                    rmindex = Some(index);
                    break;
                    //return Some(remove_id)
                }
            }
            match rmindex {
                Some(index) => {
                    self.processes.remove(index);
                    Some(index as u64)
                },
                None => None,
            }
        }
    }


    fn current(&self) -> Vec<u64> {
        let mut vec = Vec::new();
        for ids in self.processes.iter().map(|process| process.get_id() as u64){
            vec.push(ids);
        }
        vec
    }
}
