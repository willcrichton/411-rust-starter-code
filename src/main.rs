#![feature(plugin)]
#![plugin(peg_syntax_ext)]

extern crate getopts;

use getopts::Options;
use std::env;
use std::path::PathBuf;

mod parse;
mod types;
mod ir;
mod codegen;
mod util;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn compile(input: &str, matches: getopts::Matches) {
    let ast = parse::parse(PathBuf::from(input));
    if matches.opt_present("dump-ast") {
        println!("{:?}", ast);
    }

    types::typecheck(&ast);
    ast.errors.check();

    let ir = ir::trans::translate(ast);
    if matches.opt_present("dump-ir") {
        println!("{:?}", ir);
    }

    let asm = codegen::translate(ir);
    if matches.opt_present("dump-asm") {
        println!("{:?}", asm);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("", "dump-ast", "print AST");
    opts.optflag("", "dump-ir", "print IR");
    opts.optflag("", "dump-asm", "print assembly");
    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => m,
        Err(f) => panic!(f.to_string())
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    compile(&input, matches);
}
