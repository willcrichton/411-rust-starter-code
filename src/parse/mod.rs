//! Parses input files into an AST.
//!
//! Due to Rust's nascent status and the fact that it's not used very often to build compilers,
//! there are no good CFG parsers like yacc/bison/etc. out there. However, there exists excellent
//! PEG (see https://en.wikipedia.org/wiki/Parsing_expression_grammar) tools, so we must write our
//! grammar in PEG form instead of CFG form. Although it is a little more work, it can be done.

use std::cell::RefCell;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use util::{symbol, errors};
use util::mark::CodeMap;
use std::clone::Clone;

pub mod ast;
mod parse_utils;

// If you want to debug compilation errors in your grammar, then manually compile the grammar via
// the peg executable and include the generated file instead of using this macro.
peg_file! parser("grammar.rustpeg");

thread_local!(
    static GENERATOR: RefCell<symbol::Generator> = RefCell::new(symbol::Generator::new()));
thread_local!(
    static ERRORS: RefCell<Option<errors::Errors>> = RefCell::new(None));

pub fn parse(input: PathBuf) -> ast::Program {
    let mut contents = String::new();
    trypanic!(trypanic!(File::open(input.clone())).read_to_string(&mut contents));
    ERRORS.with(|errors| {
        *errors.borrow_mut() = Some(errors::Errors::new(CodeMap::new(contents.clone(), input)));
    });
    let stmts = trypanic!(parser::program(&contents));
    GENERATOR.with(|generator| (*generator.borrow()).clone().store());
    let mut errors = None;
    ERRORS.with(|errors_global| { errors = (*errors_global.borrow()).clone() });
    ast::Program {
        statements: stmts,
        errors: errors.unwrap(),
    }
}
