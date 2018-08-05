//! L1 compiler toplevel

#![feature(plugin)]
#![plugin(rustlex)]
#[allow(plugin_as_library)] extern crate rustlex;

extern crate getopts;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::Path;

use getopts::Options;

mod util;
mod parse;
mod types;
mod middle;
mod codegen;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("", "dump-ast", "print AST");
    opts.optflag("", "dump-ir", "print IR");
    opts.optflag("", "dump-asm", "print assembly");
    opts.optflag("t", "only-typecheck", "stop the compiler at typechecking");

    let matches = opts.parse(&args[1..]).unwrap();
    if matches.opt_present("h") || matches.free.is_empty() {
        print_usage(&program, opts)
    } else {
        compile(&matches.free[0], &matches)
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief))
}

/// `compile` is the main function for taking a program through the various
/// stages of compilation.
fn compile(input: &str, matches: &getopts::Matches) {
    let path = Path::new(input);
    let ast = handle_error(parse::parse(path));
    if matches.opt_present("dump-ast") {
        println!("{}", ast);
    }

    types::typecheck(&ast);
    ast.errors.check();
    if matches.opt_present("only-typecheck") {
        return;
    }

    let ir = middle::translate(ast);
    if matches.opt_present("dump-ir") {
        println!("{}", ir);
    }

    let asm = codegen::translate(ir);
    let asm = asm.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let asm = asm.connect("\n");
    if matches.opt_present("dump-asm") {
        println!("{}", asm);
    }

    handle_error(File::create(path.with_extension("s")).and_then(|mut f| {
        f.write_all(asm.as_bytes())
    }));
}

fn handle_error<T>(t: io::Result<T>) -> T {
    t.unwrap_or_else(|e| {
        panic!("I/O error: {}", e)
    })
}
