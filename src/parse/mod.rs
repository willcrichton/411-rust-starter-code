//! Parses input files into an AST

use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::Path;

use util::{Errors, CodeMap, SymbolGenerator};

pub mod ast;
mod parse_utils;

// If you want to debug compilation errors in your grammar, then manually
// compile the grammar via the peg executable and include the generated file
// instead of using this macro.
peg_file! parser("grammar.rustpeg");

thread_local! {
    static GENERATOR: RefCell<SymbolGenerator> = RefCell::new(SymbolGenerator::new())
}
thread_local! {
    static ERRORS: RefCell<Option<Errors>> = RefCell::new(None)
}

pub fn parse(input: &Path) -> io::Result<ast::Program> {
    let mut contents = String::new();
    try!(File::open(input).and_then(|mut f| f.read_to_string(&mut contents)));
    ERRORS.with(|errors| {
        let input = input.to_owned();
        *errors.borrow_mut() = Some(Errors::new(CodeMap::new(contents.clone(), input)));
    });
    let stmts = parser::program(&contents).unwrap();
    GENERATOR.with(|generator| (*generator.borrow()).clone().store());
    Ok(ast::Program {
        statements: stmts,
        errors: ERRORS.with(|errors| errors.borrow_mut().take().unwrap()),
    })
}
