//! Symbol-related functionality
//!
//! This module contains functions and helpers related to generating and
//! managing symbols in a program. Symbols are represented as indices into a
//! vector allocated elsewhere in order to allow symbols to be copyable and to
//! deduplicate copies of symbols in a program.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

/// A symbol, represented as a pointer into a table elsewhere.
///
/// Symbols can be compared for equality and inequality, as well as hashed to be
/// keys later in hash maps.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub struct Symbol(usize);

/// A structure to help generate symbols.
///
/// This structure is used during lexing and parsing to generate identifiers in
/// the AST.
#[derive(Clone)]
pub struct Generator {
    symbols: Vec<String>,
    table: HashMap<String, usize>,
}

thread_local!(static SYMBOLS: RefCell<Vec<String>> = RefCell::new(Vec::new()));

impl Generator {
    /// Creates a new empty symbol generator ready to generate new symbols.
    pub fn new() -> Generator {
        Generator { symbols: Vec::new(), table: HashMap::new() }
    }

    /// Interns a new symbol, returning the corresponding symbol.
    ///
    /// This will aggressively deduplicate symbols, returning a Symbol which
    /// points to a previously allocated identifer if one exists.
    pub fn intern(&mut self, s: &str) -> Symbol {
        let mut key = String::new();
        key.push_str(s);
        match self.table.get(&key) {
            Some(&i) => return Symbol(i),
            None => {}
        }
        let ret = self.symbols.len();
        self.table.insert(s.to_string(), ret);
        self.symbols.push(s.to_string());
        return Symbol(ret)
    }

    /// Consume ownership of this Generator, storing the symbol table in
    /// task-local-data.
    ///
    /// The symbol table is stored in a "global" location so all functions have
    /// access to it instead of requiring it to be passed around.
    pub fn store(self) {
        SYMBOLS.with(|symbols| {
            *symbols.borrow_mut() = self.symbols.clone();
        })
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Symbol(u) = *self;
        SYMBOLS.with(|symbols| {
            symbols.borrow()[u].fmt(f)
        })
    }
}
