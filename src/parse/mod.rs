use std::cell::RefCell;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use util::{symbol, errors, mark};
use util::mark::CodeMap;
use util::errors::Errors;
use std::clone::Clone;

pub mod ast;
mod parse_utils;
// peg_file! parser("grammar.rustpeg");
mod parser;

thread_local!(
    static GENERATOR: RefCell<symbol::Generator> = RefCell::new(symbol::Generator::new()));
thread_local!(
    static ERRORS: RefCell<Option<errors::Errors>> = RefCell::new(None));

#[macro_export]
macro_rules! trypanic {
    ($expr:expr) => (match $expr {
        Ok(val) => val,
        Err(err) => panic!(err)
    })
}


pub fn parse(input: PathBuf) -> ast::Program {
    let mut contents = String::new();
    trypanic!(trypanic!(File::open(input.clone())).read_to_string(&mut contents));
    ERRORS.with(|errors| {
        *errors.borrow_mut() = Some(Errors::new(CodeMap::new(contents.clone(), input)));
    });
    let stmts = trypanic!(parser::program(&contents));
    GENERATOR.with(|generator| (*generator.borrow()).clone().store());
    let mut errors = None;;
    ERRORS.with(|errors_global| { errors = (*errors_global.borrow()).clone() });
    ast::Program {
        statements: stmts,
        errors: errors.unwrap(),
    }
}
