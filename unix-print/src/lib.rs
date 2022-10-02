#![no_std]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
use sc::syscall;


/// Should be private but needs to be exposed for the macros to work
pub const __STDOUT: usize = 1;

/// Should be private but needs to be exposed for the macros to work
pub const __STDERR: usize = 2;

/// Corresponds to std's `print!`-macro
#[macro_export]
macro_rules! unix_print {
    ($($arg:tt)*) => {
        let mut __unix_print_writer = $crate::UnixWriter::stdout();
        let _ = core::fmt::Write::write_fmt(&mut __unix_print_writer, format_args!($($arg)*));
    }
}

/// Corresponds to std's `println!`-macro
#[macro_export]
macro_rules! unix_println {
    () => {
        let _ = $crate::__write_to_handle($crate::__STDOUT, "\n".as_bytes());
    };
    ($($arg:tt)*) => {
        let mut __unix_print_writer = $crate::UnixWriter::stdout();
        let _ = core::fmt::Write::write_fmt(&mut __unix_print_writer, format_args!($($arg)*));
        let _ = __unix_print_writer.write_newline();

    }
}

/// Corresponds to std's `eprint!`-macro
#[macro_export]
macro_rules! unix_eprint {
    ($($arg:tt)*) => {
        let mut __unix_print_writer = $crate::UnixWriter::stderr();
        let _ = core::fmt::Write::write_fmt(&mut __unix_print_writer, format_args!($($arg)*));
    }
}

/// Corresponds to std's `eprintln!`-macro
#[macro_export]
macro_rules! unix_eprintln {
    () => {
        let _ = $crate::__write_to_handle($crate::__STDERR, "\n".as_bytes());
    };
    ($($arg:tt)*) => {
        let mut __unix_print_writer = $crate::UnixWriter::stderr();
        let _ = core::fmt::Write::write_fmt(&mut __unix_print_writer, format_args!($($arg)*));
        let _ = __unix_print_writer.write_newline();

    }
}

/// Corresponds to std's `dbg!`-macro
#[macro_export]
macro_rules! unix_dbg {
    () => {
        $crate::unix_eprintln!("[{}:{}]", core::file!(), core::line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                $crate::unix_eprintln!("[{}:{}] {} = {:#?}",
                    core::file!(), core::line!(), core::stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}

/// Should be private but needs to be exposed for the macro to work
#[allow(clippy::cast_possible_wrap)]
#[must_use]
pub fn __write_to_handle(fd: usize, msg: &[u8]) -> isize {
    unsafe {
        syscall!(WRITE, fd, msg.as_ptr(), msg.len()) as isize
    }
}

#[allow(clippy::cast_sign_loss)]
fn try_print(fd: usize, msg: &str) -> core::fmt::Result {
    let buf = msg.as_bytes();
    let len = buf.len();
    let mut flushed = 0;
    loop {
        let res = __write_to_handle(fd, &buf[flushed..]);
        match res.cmp(&0) {
            core::cmp::Ordering::Less => return Err(core::fmt::Error),
            core::cmp::Ordering::Equal => return Ok(()),
            core::cmp::Ordering::Greater => {
                // Greater than zero
                flushed += res as usize;
                if flushed >= len {
                    return Ok(())
                }
            }
        }
    }
}

pub struct UnixWriter(usize);

impl UnixWriter {
    #[must_use]
    pub fn stdout() -> Self {
        Self(__STDOUT)
    }

    #[must_use]
    pub fn stderr() -> Self {
        Self(__STDERR)
    }

    /// # Errors
    /// Will return an error if the underlying syscall fails
    pub fn write_newline(&self) -> core::fmt::Result {
        try_print(self.0, "\n")
    }
}

impl core::fmt::Write for UnixWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        try_print(self.0, s)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prints() {
        unix_println!("-- First");
        unix_print!("My first");
        unix_print!(" two messages");
        unix_print!(" were cutoff but it's fine");
        unix_println!();
        unix_println!("-- Second\nHello there {}", "me");
    }

    #[test]
    fn test_eprints() {
        unix_eprintln!("-- First");
        unix_eprint!("My first");
        unix_eprint!(" two messages");
        unix_eprint!(" were cutoff but it's fine");
        unix_eprintln!();
        unix_eprintln!("-- Second\nHello there {}", "me");
    }

    #[test]
    fn test_dbgs() {
        unix_dbg!();
        let val = 5;
        let res = unix_dbg!(val) - 5;
        assert_eq!(0, res);
    }

}
