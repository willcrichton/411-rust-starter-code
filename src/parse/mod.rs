//! Parses input files into an AST

use std::cell::RefCell;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use util::errors::Errors;
use util::mark::CodeMap;
use util::symbol::Generator;
use std::clone::Clone;

pub mod ast;
mod parse_utils;

// If you want to debug compilation errors in your grammar, then manually compile the grammar via
// the peg executable and include the generated file instead of using this macro.
peg_file! parser("grammar.rustpeg");

thread_local!(
    static GENERATOR: RefCell<Generator> = RefCell::new(Generator::new()));
thread_local!(
    static ERRORS: RefCell<Option<Errors>> = RefCell::new(None));

pub fn parse(input: PathBuf) -> ast::Program {
    let mut contents = String::new();
    File::open(input.clone()).unwrap().read_to_string(&mut contents).unwrap();
    ERRORS.with(|errors| {
        *errors.borrow_mut() = Some(Errors::new(CodeMap::new(contents.clone(), input)));
    });
    let stmts = parser::program(&contents).unwrap();
    GENERATOR.with(|generator| (*generator.borrow()).clone().store());
    let mut errors = None;
    ERRORS.with(|errors_global| { errors = (*errors_global.borrow()).clone() });
    ast::Program {
        statements: stmts,
        errors: errors.expect("Failed to get parser errors"),
    }
}
