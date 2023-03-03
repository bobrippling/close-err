//! Exposes a `close` method on files, to permit finer grained error handling.
//!
//! ```
//! use close_err::Closable;
//! use std::{fs::File, io::Write};
//!
//! let mut f = File::create("abc.txt").unwrap();
//! f.write_all("hello".as_bytes());
//! f.close().unwrap();
//! ```

use std::io;

pub trait Closable {
    fn close(self) -> Result<(), io::Error>;
}

#[cfg(unix)]
impl<T> Closable for T
where
    T: std::os::unix::io::IntoRawFd,
{
    fn close(self) -> Result<(), io::Error> {
        let fd = self.into_raw_fd();
        let rc = unsafe { libc::close(fd) };
        bool_to_last_error(rc == 0)
    }
}

#[cfg(windows)]
impl<T> Closable for T
where
    T: std::os::windows::io::IntoRawHandle,
{
    fn close(self) -> Result<(), io::Error> {
        let handle = self.into_raw_handle();
        let rc = unsafe { kernel32::CloseHandle(handle) };
        bool_to_last_error(rc != 0)
    }
}

fn bool_to_last_error(b: bool) -> Result<(), io::Error> {
    b
        .then_some(())
        .ok_or_else(io::Error::last_os_error)
}
