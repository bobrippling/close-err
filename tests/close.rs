use std::{io, fs::File};

use close_err::Closable;

#[test]
fn on_fs() {
    fn explicit_close() -> Result<(), io::Error> {
        {
            let f = File::create("hi.txt")?;

            f.close()?;
        }

        assert_removed("hi.txt");

        Ok(())
    }

    fn implicit_close() -> Result<(), io::Error> {
        {
            let _ = File::create("hi.txt")?;
        }

        assert_removed("hi.txt");

        Ok(())
    }

    explicit_close().unwrap();
    implicit_close().unwrap();
}

fn assert_removed(path: &str) {
    use std::fs;

    fs::remove_file(path).unwrap();
}
