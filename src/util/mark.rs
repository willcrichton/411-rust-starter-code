//! Positional markers
//!
//! This module contains an implementation of positional markers to attribute
//! nodes in the AST with information from whence they came. This facilitates
//! higher quality error messages.

use std::fmt;
use std::path::PathBuf;
use std::borrow::Borrow;

/// A mark is represented by the (lo, hi) byte offsets into the original source
/// program.
///
/// It is guaranteed that `lo <= hi`. A `CodeMap` instance is needed to make
/// sense of a `Mark`.
#[derive(Clone, Eq, PartialEq)]
pub struct Mark { lo: u32, hi: u32, }

/// A generic wrapper to contain a marked piece of information.
///
/// This is convenient for specifying that the inner `node` is produced by the
/// code mentioned by `mark`.
#[derive(Clone, Eq)]
pub struct Marked<T> {
    pub mark: Mark,
    pub node: T,
}

/// Representation of a source program to translate a `Mark` to a `String`.
#[derive(Clone)]
pub struct CodeMap {
    code: String,
    file: PathBuf,
}

/// A dummy span to represent the "entire program"
pub static DUMMY: Mark = Mark { lo: 0, hi: 0 };

impl Mark {
    /// Creates a new `Mark` which is bounded by `lo` and `hi` in the source
    /// code of the original program.
    pub fn new(lo: u32, hi: u32) -> Mark {
        Mark { lo: lo, hi: hi }
    }

    /// Converts this `Mark` to a string given the specified code map.
    pub fn to_string(&self, cm: &CodeMap) -> String {
        let (loline, locol) = cm.linecol(self.lo);
        let (hiline, hicol) = cm.linecol(self.hi);
        format!("{}:{}-{}:{}", loline, locol, hiline, hicol)
    }
}

impl<T> Marked<T> {
    /// Helper function for creating a new instance of a marked node.
    pub fn new(t: T, mark: Mark) -> Marked<T> {
        Marked { node: t, mark: mark }
    }

    /// Unwrap the inner value contained within this marked node.
    pub fn unwrap(self) -> T { self.node }
}

/// Allow marked instances to be compared with == and !=
impl<T: PartialEq> PartialEq for Marked<T> {
    fn eq(&self, other: &Marked<T>) -> bool { self.node.eq(&other.node) }
}

/// Allow marked things to be printed with `{:?}`
impl<T: fmt::Display> fmt::Display for Marked<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.node.fmt(f)
    }
}

impl CodeMap {
    /// Creates a new code map for the program in question.
    pub fn new(code: String, file: PathBuf) -> CodeMap {
        CodeMap { code: code, file: file }
    }

    /// Converts a bytes offset of a `Mark` into a (line, column) pair.
    ///
    /// All indexes are 1-based.
    pub fn linecol(&self, offset: u32) -> (usize, usize) {
        let offset = offset as usize;
        let mut cur = 0;
        let code_str: &str = self.code.borrow();
        for (i, line) in code_str.lines().enumerate() {
            if cur + line.len() > offset {
                return (i + 1, offset - cur + 1)
            }
            cur += line.len() + 1;
        }
        (code_str.lines().count() + 1, 0 + 1)
    }

    /// Returns the file that this code map represents.
    pub fn file(&self) -> &PathBuf { &self.file }
}
