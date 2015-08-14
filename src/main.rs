//! L1 compiler toplevel

#![feature(plugin)]
#![plugin(peg_syntax_ext)]

extern crate getopts;

use getopts::Options;
use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

mod util;
mod parse;
mod types;
mod ir;
mod codegen;

/// `compile` is the main function for taking a program through the various stages of compilation.
fn compile(input: &str, matches: getopts::Matches) {
    let mut path = PathBuf::from(input);
    let ast = parse::parse(path.clone());
    if matches.opt_present("dump-ast") {
        println!("{}", ast);
    }

    types::typecheck(&ast);
    ast.errors.check();

    let ir = ir::translate(ast);
    if matches.opt_present("dump-ir") {
        println!("{}", ir);
    }

    let asm = codegen::translate(ir);
    let asm: Vec<String> = asm.into_iter().map(|x| format!("{}", x)).collect();
    let asm = asm.join("\n");
    if matches.opt_present("dump-asm") {
        println!("{}", asm);
    }

    path.set_extension("S");
    let mut outfile = File::create(path).unwrap();
    outfile.write_all(asm.as_bytes()).unwrap();
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("", "dump-ast", "print AST");
    opts.optflag("", "dump-ir", "print IR");
    opts.optflag("", "dump-asm", "print assembly");
    
    let matches = opts.parse(&args[1..]).unwrap();
    if matches.opt_present("h") || matches.free.is_empty() {
        print_usage(&program, opts);
        return;
    }

    compile(&matches.free[0].clone(), matches);
}
