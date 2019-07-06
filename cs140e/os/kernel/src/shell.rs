use console::{kprint, kprintln, CONSOLE};
use fat32::traits::{Dir, Entry, File, FileSystem, Metadata, Timestamp};
use std::io::{SeekFrom, Seek, Read};
use std::path::PathBuf;
use std::str;
use FILE_SYSTEM;
#[cfg(not(test))]
use ALLOCATOR;
use SCHEDULER;

use process::{sys_sleep,sys_kill};

const BANNER: &str = r#"
 _               _
| |   _   _  ___| | ___   _
| |  | | | |/ __| |/ / | | |
| |__| |_| | (__|   <| |_| |
|_____\__,_|\___|_|\_\\__, |
                      |___/
"#;

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs,
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: Vec<&'a str>,
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &str) -> Result<Command, Error> {
        let mut args = Vec::with_capacity(MAXARG);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            if args.len() == args.capacity() {
                return Err(Error::TooManyArgs);
            } else {
                args.push(arg);
            }
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        self.args[0]
    }

    #[allow(dead_code)]
    pub fn execute(&self) {
        // unused function
        unimplemented!();
    }
}

const GR: u8 = 0x0A;
// \r
const GN: u8 = 0x0D;
// \n
const BS: u8 = 0x08;
// backspace
const DE: u8 = 0x7F;
// delete
const BE: u8 = 0x07;
// ring the bell
const ESC: u8 = 0x1B;
const CTRL_A: u8 = 0x01;
const CTRL_B: u8 = 0x02;
const CTRL_D: u8 = 0x04;
const CTRL_E: u8 = 0x05;
const CTRL_F: u8 = 0x06;
const CTRL_K: u8 = 0x0b;
const CTRL_N: u8 = 0x0e;
const CTRL_P: u8 = 0x10;
const CTRL_U: u8 = 0x15;

/// Read a command from users input
fn readcmd(history: &mut Vec<Vec<u8>>) -> String {
    let mut buf = Vec::with_capacity(MAXBUF);
    let mut cursor = 0;
    let mut which_hist = history.len();

    loop {
        let byte = CONSOLE.lock().read_byte();
        match byte {
            GR | GN => {
                // finish input
                kprintln!("");
                break;
            }
            BS | DE if cursor > 0 => {
                // delete a char
                cursor -= 1;
                buf.remove(cursor);
                kprint!("{}", BS as char);
                for ch in &buf[cursor..] {
                    kprint!("{}", *ch as char);
                }
                kprint!(" ");
                for _ in cursor..=buf.len() {
                    kprint!("{}", BS as char);
                }
            }
            _ if buf.len() == buf.capacity() => {
                // ring the bell if overflow
                kprint!("{}", BE as char);
            }
            byte @ b' '...b'~' => {
                // normal char
                buf.insert(cursor, byte);
                for ch in &buf[cursor..] {
                    kprint!("{}", *ch as char);
                }
                cursor += 1;
                for _ in cursor..buf.len() {
                    kprint!("{}", BS as char);
                }
            }
            CTRL_A => {
                // move cursor to start
                for _ in 0..cursor {
                    kprint!("{}", BS as char);
                }
                cursor = 0;
            }
            CTRL_B => {
                // move cursor back
                if cursor > 0 {
                    kprint!("{}", BS as char);
                    cursor -= 1;
                }
            }
            CTRL_D => {
                // delete
                if cursor == buf.len() {
                    continue;
                }
                buf.remove(cursor);
                for ch in &buf[cursor..] {
                    kprint!("{}", *ch as char);
                }
                kprint!(" {}", BS as char);
                for _ in cursor..buf.len() {
                    kprint!("{}", BS as char);
                }
            }
            CTRL_E => {
                // move cursor to the end
                for _ in cursor..buf.len() {
                    kprint!("{}{}{}", ESC as char, '[', 'C');
                }
                cursor = buf.len();
            }
            CTRL_F => {
                // move cursor forward
                if cursor < buf.len() {
                    kprint!("{}{}{}", ESC as char, '[', 'C');
                    cursor += 1;
                }
            }
            CTRL_K => {
                kprint!("{}[K", ESC as char);
                for _ in cursor..buf.len() {
                    buf.remove(cursor);
                }
            }
            CTRL_N => {
                // next history
                if which_hist + 1 < history.len() {
                    which_hist += 1;
                } else {
                    continue;
                }

                for _ in 0..buf.len() {
                    kprint!("{} {}", BS as char, BS as char);
                }

                buf = history[which_hist].clone();
                let tmp = buf.len();
                buf.reserve(MAXBUF - tmp);
                for ch in &buf[..] {
                    kprint!("{}", *ch as char);
                }
                cursor = buf.len();
            }
            CTRL_P => {
                // previous history
                if which_hist > 0 {
                    which_hist -= 1;
                } else {
                    continue;
                }

                for _ in 0..buf.len() {
                    kprint!("{} {}", BS as char, BS as char);
                }

                buf = history[which_hist].clone();
                let tmp = buf.len();
                buf.reserve(MAXBUF - tmp);
                for ch in &buf[..] {
                    kprint!("{}", *ch as char);
                }
                cursor = buf.len();
            }
            CTRL_U => {
                // clear line
                for _ in cursor..buf.len() {
                    kprint!("{}{}{}", ESC as char, '[', 'C');
                }
                for _ in 0..buf.len() {
                    buf.remove(0);
                    kprint!("{} {}", BS as char, BS as char);
                }
                cursor = 0;
            }
            ESC => {
                let ch = CONSOLE.lock().read_byte();
                if ch != b'[' {
                    kprint!("{}", BE as char);
                    continue;
                }
                let ch = CONSOLE.lock().read_byte();
                match ch {
                    b'D' => {
                        // left
                        if cursor > 0 {
                            kprint!("{}", BS as char);
                            cursor -= 1;
                        }
                    }
                    b'C' => {
                        // right
                        if cursor < buf.len() {
                            kprint!("{}{}{}", ESC as char, '[', 'C');
                            cursor += 1;
                        }
                    }
                    up_down @ b'A' | up_down @ b'B' => {
                        if up_down == b'A' && which_hist > 0 {
                            // up
                            which_hist -= 1;
                        } else if up_down == b'B' && which_hist + 1 < history.len() {
                            // down
                            which_hist += 1;
                        } else {
                            kprint!("{}", BE as char);
                            continue;
                        }

                        // clear line
                        for _ in 0..buf.len() {
                            kprint!("{} {}", BS as char, BS as char);
                        }

                        buf = history[which_hist].clone();
                        let tmp = buf.len();
                        buf.reserve(MAXBUF - tmp);
                        for ch in &buf[..] {
                            kprint!("{}", *ch as char);
                        }
                        cursor = buf.len();
                    }
                    _ => {
                        kprint!("{}", BE as char);
                    }
                }
            }
            _ => {
                // unrecognized char
                kprint!("{}", BE as char);
            }
        }
    }

    history.push(buf.clone());
    String::from_utf8(buf).unwrap_or_default()
}

const MAXBUF: usize = 512;
const MAXARG: usize = 64;

/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str) -> ! {
    kprintln!("{}", BANNER);

    let mut cwd = PathBuf::from("/");
    let mut history = Vec::new();

    loop {
        kprint!("({}) {}", cwd.display(), prefix);
        match Command::parse(&readcmd(&mut history)) {
            Ok(cmd) => match cmd.path() {
                "echo" => command_echo(&cmd),
                "pwd" => command_pwd(&cmd, &cwd),
                "cd" => command_cd(&cmd, &mut cwd),
                "ls" => command_ls(&cmd, &cwd),
                "cat" => command_cat(&cmd, &cwd),
                "allocator" => command_allocator(&cmd),
                "sleep" => command_sleep(&cmd),
                "kill" => command_kill(&cmd),
                _ => {
                    kprintln!("unknown command: {}", cmd.path());
                }
            },
            Err(Error::TooManyArgs) => kprintln!("error: too many arguments"),
            Err(Error::Empty) => {}
        }
    }
}

fn command_allocator(_cmd: &Command) {
    #[cfg(not(test))]
    kprintln!("{:?}", ALLOCATOR);
}

fn command_echo(cmd: &Command) {
    let length = cmd.args.len();
    for i in 1..length {
        kprint!("{} ", cmd.args.as_slice()[i]);
    }
    kprintln!("");
}

fn command_pwd(_cmd: &Command, cwd: &PathBuf) {
    kprintln!("{}", cwd.display());
}

fn command_cd(cmd: &Command, cwd: &mut PathBuf) {
    if cmd.args.len() != 2 {
        kprintln!("Wrong syntax for cd");
        return;
    }
    let tmp = cwd.clone();
    let new_path = FILE_SYSTEM
        .canonicalize(tmp.join(cmd.args[1]))
        .unwrap_or(tmp);
    if let Ok(_) = FILE_SYSTEM.open_dir(&new_path) {
        *cwd = new_path;
    } else {
        kprintln!("Error: not a directory");
    }
}

fn command_ls(cmd: &Command, cwd: &PathBuf) {
    let (cfg_all, path) = match cmd.args.len() {
        1 => (false, PathBuf::from(cwd)),
        2 => {
            if cmd.args[1] == "-a" {
                (true, PathBuf::from(cwd))
            } else {
                (false, PathBuf::from(cwd.join(cmd.args[1])))
            }
        }
        3 => {
            if cmd.args[1] == "-a" {
                (true, PathBuf::from(cmd.args[2]))
            } else if cmd.args[2] == "-a" {
                (true, PathBuf::from(cwd.join(cmd.args[1])))
            } else {
                kprintln!("Wrong syntax for ls");
                return;
            }
        }
        _ => {
            kprintln!("Wrong syntax for ls");
            return;
        }
    };

    let good_path = FILE_SYSTEM.canonicalize(path).unwrap();

    let dir = match FILE_SYSTEM.open_dir(good_path) {
        Ok(thing) => thing,
        Err(e) => {
            kprintln!("Error opening dir: {}", e);
            return;
        }
    };

    match dir.entries() {
        Ok(entries) => {
            for entry in entries {
                let metadata = entry.metadata();
                if !cfg_all && (metadata.hidden() || entry.name() == "." || entry.name() == "..") {
                    continue;
                }
                kprint!(
                    "{}{}{}\t",
                    if entry.is_dir() { 'd' } else { '-' },
                    if metadata.read_only() { 'r' } else { 'w' },
                    if metadata.hidden() { 'h' } else { 'v' },
                );
                let md_cr = metadata.created();
                let md_mo = metadata.modified();
                kprint!(
                    "{:0>4}/{:0>2}/{:0>2} {:0>2}:{:0>2}:{:0>2}\t{:0>4}/{:0>2}/{:0>2} {:0>2}:{:0>2}:{:0>2}\t",
                    md_cr.year(),
                    md_cr.month(),
                    md_cr.day(),
                    md_cr.hour(),
                    md_cr.minute(),
                    md_cr.second(),
                    md_mo.year(),
                    md_mo.month(),
                    md_mo.day(),
                    md_mo.hour(),
                    md_mo.minute(),
                    md_mo.second(),
                );
                if entry.is_dir() {
                    kprintln!("{}\t{}/", 0, entry.name())
                } else {
                    kprintln!("{}\t{}", entry.as_file().unwrap().size(), entry.name());
                }
            }
        }
        Err(e) => kprintln!("Error listing dir: {}", e),
    }
}

fn command_cat(cmd: &Command, cwd: &PathBuf) {
    'outer: for path in &cmd.args[1..] {
        let good_path = FILE_SYSTEM.canonicalize(cwd.join(path)).unwrap();
        let mut file = match FILE_SYSTEM.open_file(good_path) {
            Ok(thing) => thing,
            Err(e) => {
                kprintln!("Error opening file: {}", e);
                return;
            }
        };

        const BUF_SIZE: usize = 1024;
        let mut buf = [0u8; BUF_SIZE];
        loop {
            match file.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => match str::from_utf8(&buf[..n]) {
                    Ok(stuff) => kprint!("{}", stuff),
                    Err(e) => {
                        let index = e.valid_up_to();
                        let padding = index as i64 - n as i64;
                        if -padding >= 4 {
                            kprintln!("Error: invalid UTF-8");
                            continue 'outer;
                        }
                        kprint!("{}", str::from_utf8(&buf[..index]).unwrap());
                        match file.seek(SeekFrom::Current(padding)) {
                            Ok(_) => {}
                            Err(_) => {
                                kprintln!("Error reading file: {}", e);
                                return;
                            }
                        }
                    }
                },
                Err(e) => {
                    kprintln!("Error reading file: {}", e);
                    return;
                }
            }
        }
    }
}

fn command_sleep(cmd: &Command) {
    match cmd.args.len() {
        1 => {
            kprintln!("Missing parameter.");
        }
        2 => {
            match cmd.args[1].parse() {
                Ok(x) => {
                    kprintln!("Pi will sleep {} ms.",x);
                    sys_sleep(x);
                }
                Err(_) => {
                    kprintln!("Wrong parameter.");
                }
            }
        }
        _ => {
            kprintln!("Too many parameters.");
        }
    }
}

fn command_kill(cmd: &Command) {
    match cmd.args.len() {
        1 => {
            kprintln!("Missing parameter.");
        }
        2 => {
            match cmd.args[1].parse() {
                Ok(x) => {
                    kprintln!("Pi will kill process {}.",x);
                    sys_kill(x);
                }
                Err(_) => {
                    kprintln!("Wrong parameter.");
                }
            }
        }
        _ => {
            kprintln!("Too many parameters.");
        }
    }
}


fn command_ps(cmd: &Command) {
    match cmd.args.len() {
        1 => {
            kprintln!("Current process: {:?}",SCHEDULER.current());
        }
        _ => {
            kprintln!("Too many parameters.");
        }
    }
}

