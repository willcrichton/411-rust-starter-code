extern crate lalrpop;

fn main() {
    // Build the lexer
    extern crate syntex;
    extern crate rustlex_codegen;
    use std::path::Path;

    let mut registry = syntex::Registry::new();
    rustlex_codegen::plugin_registrar(&mut registry);
    let src = Path::new("src/parse/lexer.rs");
    let dst = Path::new("src/parse/lexer_generated.rs");
    registry.expand("", &src, &dst).unwrap();

    // Build the parser
    lalrpop::process_root_unconditionally().unwrap();
}
