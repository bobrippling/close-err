//! Exposes a `close` method on files, to permit finer grained error handling.
//!
//! ```
//! use close_err::Closable;
//! use std::{fs::File, io::Write};
//!
//! let mut f = File::create("abc").unwrap();
//! f.write_all("hello".as_bytes());
//! f.close();
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
        let success = {
            let fd = self.into_raw_fd();
            let rc = unsafe { libc::close(fd) };
            rc == 0
        };

        if success {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

#[cfg(windows)]
impl<T> Closable for T
where
    T: std::os::windows::io::IntoRawHandle,
{
    fn close(self) -> Result<(), io::Error> {
        let success = {
            let handle = self.into_raw_handle();
            let rc = unsafe { kernel32::CloseHandle(handle) };
            rc != 0
        };

        if success {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }
}
