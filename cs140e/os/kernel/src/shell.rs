use console::{kprint, kprintln, CONSOLE};
use fat32::traits::{Dir, Entry, File, FileSystem, Metadata, Timestamp};
use pi::atags::Atags;
use stack_vec::StackVec;
use std::io::{SeekFrom, Seek, Read};
use std::path::PathBuf;
use std::str;
use FILE_SYSTEM;
#[cfg(not(test))]
use ALLOCATOR;

const BANNER: &str = r#"
 ____
 | __ )  __ _ _ __  _ __   ___ _ __
 |  _ \ / _` | '_ \| '_ \ / _ \ '__|
 | |_) | (_| | | | | | | |  __/ |
 |____/ \__,_|_| |_|_| |_|\___|_|
"#;

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs,
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>,
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
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

const GR: u8 = 0x0A;   // \r
const GN: u8 = 0x0D;   // \n
const BS: u8 = 0x08;   // backspace
const DE: u8 = 0x7F;   // delete
const BE: u8 = 0x07;   // ring the bell
const CTRL_U: u8 = 0x15;
/// Read a command from users input
fn readcmd(buf: &mut [u8]) -> &str {
    let mut len = 0;
    loop {
        let byte = CONSOLE.lock().read_byte();
        match byte {
            GR | GN => {
                // finish input
                kprintln!("");
                break;
            }
            BS | DE if len > 0 => {
                // delete a char
                kprint!("{} {}", BS as char, BS as char);
                len -= 1;
            }
            _ if len == buf.len() => {
                // ring the bell if overflow
                kprint!("{}", BE as char);
            }
            byte @ b' '...b'~' => {
                // normal char
                kprint!("{}", byte as char);
                buf[len] = byte;
                len += 1;
            }
            CTRL_U => {
                // clear line
                for i in 0..len {
                    buf[i] = 0;
                    kprint!("{} {}", BS as char, BS as char);
                }
                len = 0;
            }
            _ => {
                // unrecognized char
                kprint!("{}", BE as char);
            }
        }
    }
    str::from_utf8(&buf[..len]).unwrap()
}

const MAXBUF: usize = 512;
const MAXARG: usize = 64;
/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str) -> ! {
    kprintln!("{}", BANNER);

    let mut cwd = PathBuf::from("/");
    loop {
        kprint!("({}) {}", cwd.display(), prefix);
        match Command::parse(readcmd(&mut [0u8; MAXBUF]), &mut [""; MAXARG]) {
            Ok(cmd) => match cmd.path() {
                "echo" => command_echo(&cmd),
                "pwd" => command_pwd(&cmd, &cwd),
                "cd" => command_cd(&cmd, &mut cwd),
                "ls" => command_ls(&cmd, &cwd),
                "cat" => command_cat(&cmd, &cwd),
                "atags" => command_atags(&cmd),
                "allocator" => command_allocator(&cmd),
                _ => {
                    kprintln!("unknown command: {}", cmd.path());
                }
            },
            Err(Error::TooManyArgs) => kprintln!("error: too many arguments"),
            Err(Error::Empty) => {}
        }
    }
}

fn command_atags(_cmd: &Command) {
    for atag in Atags::get() {
        kprintln!("{:?}", atag);
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
    for path in &cmd.args[1..] {
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
                            return;
                        }
                        kprint!("{}", str::from_utf8(&buf[..index]).unwrap());
                        file.seek(SeekFrom::Current(padding)).unwrap();
                    }
                },
                Err(e) => kprintln!("Error reading file: {}", e),
            }
        }
    }
}
