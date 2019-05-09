use stack_vec::StackVec;
use std::str;
use console::{kprint, kprintln, CONSOLE};

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>
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

    pub fn execute(&self) {
        match self.path() {
            "echo" => {
                let length = self.args.len();
                for i in 1..(length-1) {
                    kprint!("{} ", self.args.as_slice()[i]);
                }
                kprintln!("{}", self.args.as_slice()[length-1]);
            }
            _ => {
                kprintln!("unknown command: {}", self.path());
            }
        }
    }
}

const GR: u8 = 0x0A; //\r
const GN: u8 = 0x0D; //\n
const BS: u8 = 0x08; //backspace
const DE: u8 = 0x7F; //delete
const BE: u8 = 0x07; //ring the bell
/// Read a command from users input
fn readcmd(buf: &mut [u8]) -> &str { 
    let mut len = 0;
    loop { 
        let byte = CONSOLE.lock().read_byte();
        match byte {
            GR | GN => {                                    //finish input
                kprintln!("");
                break;
            }
            BS | DE if len > 0 => {                         //delete a char
                kprint!("{} {}", BS as char, BS as char);
                len -= 1;
            }
            _ if len == buf.len() => {                        //ring the bell if overflow
                kprint!("{}", BE as char);
            }
            byte @ b' ' ... b'~' => {                       //normal char
                kprint!("{}", byte);
                buf[len] = byte;
                len += 1;
            }
            _ => {                                             //unrecognized char
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
    loop {
        kprint!("{} ", prefix);
        match Command::parse(readcmd(&mut [0u8; MAXBUF]), &mut [""; MAXARG]) {
            Ok(cmd) => cmd.execute(),
            Err(Error::TooManyArgs) => kprintln!("error: too many arguments"),
            Err(Error::Empty) => { }
        }
    }
}
