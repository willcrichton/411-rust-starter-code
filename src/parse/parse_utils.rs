//! Various functions to aid the hacky PEG parser

use util::mark::{Mark, Marked};
use super::{GENERATOR, ERRORS, ast};

pub fn mark<T>(obj: T, lo: usize, hi: usize) -> Marked<T> {
    Marked::new(obj, Mark::new(lo as u32, hi as u32))
}

fn error(s: String, lo: usize, hi: usize) -> ! {
    ERRORS.with(|errors| {
        errors.borrow().as_ref().expect("Parser errors struct not created")
            .die(&Mark::new(lo as u32, hi as u32), s);
    });
    unreachable!()
}

pub fn str_to_ident(s: &str) -> ast::Ident {
    let mut symbol = None;
    GENERATOR.with(|generator| symbol = Some(generator.borrow_mut().intern(s)));
    symbol.expect("Symbol generator failed")
}

// TODO(wcrichto): check somewhere for signed integers > 2^31
pub fn parse_number(s: &str, base: u32, lo: usize, hi: usize) -> u32 {
    match u32::from_str_radix(s, base) {
        Ok(n) => n,
        Err(err) => error(format!("{} is an invalid integer: {}", s, err), lo, hi)
    }
}

pub fn vec_to_expr(e: ast::Expression, vec: Vec<(ast::Operator, ast::Expression)>,
                   lo: usize, hi: usize) -> ast::Expression {
    let mut vec = vec;
    let (op, e2) = vec.pop().expect("Empty vec passed to vec_to_expr");
    let e_vec = 
        if vec.len() == 0 {
            ast::Expression_::Binary(op, Box::new(e), Box::new(e2))
        } else {
            ast::Expression_::Binary(
                op, Box::new(e), Box::new(vec_to_expr(e2, vec, lo, hi)))
        };
    mark(e_vec, lo, hi)
}
