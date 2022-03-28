# close-err - handle errors raised when closing file-like types

Exposes a `close` method on files (and similar types), to permit finer grained error handling.

Inspired by [`close-file`], but more generic and works correctly on unix.

[`close-file`]: https://crates.io/crates/close-file
