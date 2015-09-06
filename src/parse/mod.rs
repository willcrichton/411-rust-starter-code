//! Parses input files into an AST

extern crate lalrpop_util;

use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::Path;
use self::lalrpop_util::ParseError;

use util::{Errors, CodeMap, SymbolGenerator, Mark, DUMMY_MARK};
use self::lexer_generated::Lexer;
use self::token::Token;

pub mod ast;
mod token;
mod lexer_generated;
mod parser;

thread_local! {
    static GENERATOR: RefCell<SymbolGenerator> =
        RefCell::new(SymbolGenerator::new())
}

thread_local! {
    static ERRORS: RefCell<Option<Errors>> = RefCell::new(None)
}

fn parser_panic(s: String, m: Mark) -> ! {
    ERRORS.with(|errors| {
        errors.borrow().as_ref().expect("Parser errors struct not created")
            .die(&m, &s);
    });
    unreachable!()
}

pub fn parse(input: &Path) -> io::Result<ast::Program> {
    let mut contents = String::new();
    try!(File::open(input).and_then(|mut f| f.read_to_string(&mut contents)));

    ERRORS.with(|errors| {
        let input = input.to_owned();
        *errors.borrow_mut() =
            Some(Errors::new(CodeMap::new(contents.clone(), input)));
    });

    let lexer = Lexer::new(io::BufReader::new(contents.as_bytes()));
    let tokens = lexer.into_iter().map(|x| (x.mark.lo, x.node, x.mark.hi))
        .collect::<Vec<(usize, Token, usize)>>();

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

    Ok(ast::Program {
        statements: stmts,
        errors: ERRORS.with(|errors| errors.borrow_mut().take().unwrap()),
    })
}
