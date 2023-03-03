# close-err - handle errors raised when closing file-like types

Exposes a `close` method on files (and similar types), to permit finer grained error handling.

Inspired by [`close-file`], but more generic and works correctly on unix.

For example:
```rust
use std::{io, fs::File};

use close_err::Closable;

fn main() -> Result<(), io::Error> {
	let f = File::create("hi.txt")?;

	f.close()?;

	Ok(())
}
```

[`close-file`]: https://crates.io/crates/close-file
