//! IR Temporaries

use std::fmt;
use std::cell::Cell;

/// A temporary in the IR.
///
/// Each temporary represents a numbered variable.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub struct Temp(usize);

/// An allocator of Temporaries, used during translation.
pub struct TempAllocator {
    next: Cell<usize>,
}

impl TempAllocator {
    /// Prepares a new allocator ready to create new temporaries
    pub fn new() -> TempAllocator { TempAllocator { next: Cell::new(0) } }

    /// Returns the number of temporaries allocated so far
    pub fn count(&self) -> usize { self.next.get() }

    /// Resets the allocator back to 0
    pub fn reset(&self) { self.next.set(0); }

    /// Generates a new unique temporary
    pub fn gen(&self) -> Temp {
        let ret = self.next.get();
        self.next.set(ret + 1);
        Temp(ret)
    }
}

impl fmt::Display for Temp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Temp(i) = *self;
        write!(f, "%t{}", i)
    }
}
