//! Parses input files into an AST

extern crate lalrpop_util;

use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::{Path, PathBuf};
use std::thread;
use std::sync::mpsc::channel;
use self::lalrpop_util::ParseError;

use util::{Errors, CodeMap, SymbolGenerator, Mark, DUMMY_MARK, Symbol};
use self::lexer::Lexer;

pub mod ast;
mod token;
mod lexer;
mod parser;

thread_local! {
    static GENERATOR: RefCell<SymbolGenerator> =
        RefCell::new(SymbolGenerator::new())
}

thread_local! {
    static ERRORS: RefCell<Option<Errors>> = RefCell::new(None)
}

pub fn parser_panic(s: String, m: Mark) -> ! {
    ERRORS.with(|errors| {
        errors.borrow().as_ref().expect("Parser errors struct not created")
            .die(&m, &s);
    });
    unreachable!()
}

pub fn intern(s: &str) -> Symbol {
    let mut symbol = None;
    GENERATOR.with(
        |generator| symbol = Some(generator.borrow_mut().intern(s)));
    symbol.unwrap()
}

// 24MB seems to handle all unreasonable and most reasonable programs
// If 64MB doesn't suffice, we'll remove the test
const STACK_SIZE: usize = 64 * 1024 * 1024;

pub fn parse(input: &Path) -> io::Result<ast::Program> {
    let mut contents = String::new();
    try!(File::open(input).and_then(|mut f| f.read_to_string(&mut contents)));

    // Rust's default stack size is relatively small, so certain tests cases
    // which are highly recursive, e.g. int x = 1 - 1 - 1 - 1 - ... will
    // overflow the stack. We can solve this by spawning a thread with a larger
    // stack. Alas, there is no way to increase the main thread stack size.
    let ((t1, r1), (t2, r2)) = (channel(), channel());
    thread::Builder::new().stack_size(STACK_SIZE).spawn(move || {
        let (contents, input): (String, PathBuf) = r1.recv().unwrap();

        ERRORS.with(|errors| {
            let input = input.to_owned();
            *errors.borrow_mut() =
                Some(Errors::new(CodeMap::new(contents.clone(), input)));
        });

        let mut lexer = Lexer::new(io::BufReader::new(contents.as_bytes()));
        let mut tokens = vec![];
        while let Some(tok) = lexer.next() {
            tokens.push((tok.mark.lo, tok.node, tok.mark.hi));
        }

        if lexer.comment_depth > 0 {
            parser_panic(String::from("Unclosed block comment"), DUMMY_MARK);
        }

        // We need to store here so error printing below can work.
        GENERATOR.with(|generator| (*generator.borrow()).clone().store());

        let stmts = parser::parse_Program(tokens).unwrap_or_else(|err| { match err {
            ParseError::UnrecognizedToken {token, expected} => match token {
                Some((lo, tok, hi)) => {
                    let err =
                        format!("Parse error: expected tokens {:?}, found token {:?}",
                                expected, tok);
                    parser_panic(err, Mark::new(lo, hi));
                },
                None => {
                    let err =
                        format!("Parse error: expected tokens {:?}, found EOF",
                                expected);
                    parser_panic(String::from(err), DUMMY_MARK);
                }
            },
            ParseError::ExtraToken {token} => {
                let (lo, tok, hi) = token;
                let err = format!("Parse error: found extra token {:?}", tok);
                parser_panic(err, Mark::new(lo, hi));
            },
            ParseError::User {error} => {
                let err = format!("Parse error: {:?}", error);
                parser_panic(err, DUMMY_MARK);
            },
        } } );

        ERRORS.with(|errors| {
            GENERATOR.with(|generator| {
                t2.send((stmts,
                         errors.borrow().clone().unwrap(),
                         generator.borrow().clone())).unwrap();
            });
        });
    }).unwrap();

    t1.send((contents, input.to_path_buf())).unwrap();
    let (stmts, errors, generator) = r2.recv().unwrap();
    generator.store();

    Ok(ast::Program {
        statements: stmts,
        errors: errors,
    })
}
