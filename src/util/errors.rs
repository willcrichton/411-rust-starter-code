//! A utility module to collect errors and print them.

use std::cell::Cell;
use std::io;
use std::process;

use util::mark::{Mark, CodeMap, DUMMY};

#[derive(Clone)]
pub struct Errors {
    cm: CodeMap,
    errored: Cell<bool>,
}

impl Errors {
    /// Creates a new structure which will track errors and print them for the
    /// code map specified.
    pub fn new(cm: CodeMap) -> Errors {
        Errors { cm: cm, errored: Cell::new(false) }
    }

    /// Emit an error for the specified `Mark` (location in the program).
    ///
    /// This does not abort compilation to allow more errors to be printed.
    pub fn add(&self, m: &Mark, msg: String) {
        use std::io::Write;
        let mut out = io::stderr();
        let msg = format!("error: {}\n", msg);;
        if *m == DUMMY {
            out.write(msg.as_bytes()).unwrap();
        } else {
            out.write(
                format!(
                    "{}:{}:{}",
                    self.cm.file().display(),
                    m.to_string(&self.cm),
                    msg)
                    .as_bytes())
                .unwrap();
        }
        self.errored.set(true);
    }

    /// Emit an error, and at the same time abort the program.
    pub fn die(&self, m: &Mark, msg: String) -> ! {
        self.add(m, msg);
        die()
    }

    /// Check to see whether an error has been emitted, and if so abort the
    /// program.
    pub fn check(&self) {
        if self.errored.get() {
            die();
        }
    }
}

fn die() -> ! {
    process::exit(1)
}
