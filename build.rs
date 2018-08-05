extern crate lalrpop;

fn main() {
    // Build the parser
    lalrpop::process_root_unconditionally().unwrap();
}
