use util::mark::{Mark, Marked};
use super::{GENERATOR, ERRORS, ast};

pub fn mark<T>(obj: T, lo: usize, hi: usize) -> Marked<T> {
    Marked::new(obj, Mark::new(lo as u32, hi as u32))
}

fn error(s: String, lo: usize, hi: usize) -> ! {
    ERRORS.with(|errors| {
        errors.borrow().as_ref().unwrap().die(&Mark::new(lo as u32, hi as u32), s);
    });
    unreachable!()
}

pub fn str_to_ident(s: &str) -> ast::Ident {
    let mut symbol = None;
    GENERATOR.with(|generator| symbol = Some(generator.borrow_mut().intern(s)));
    symbol.unwrap()
}

pub fn int_to_num(s: &str, lo: usize, hi: usize) -> u32{
    match s.parse::<u32>() {
        Ok(n) => {
            if n > 1 << 31 {
                error(format!("{} is an invalid integer", s), lo, hi)
            }
            return n;
        },
        Err(err) =>
            error(format!("{} is an invalid integer: {}", s, err), lo, hi)
    }
}

pub fn hex_to_num(num: &str, lo: usize, hi: usize) -> u32 {
    num.parse::<u32>().unwrap()
}

pub fn vec_to_expr(e: ast::Expression, vec: Vec<(ast::Operator, ast::Expression)>,
                   lo: usize, hi: usize) -> ast::Expression {
    let mut vec = vec;
    let (op, e2) = vec.pop().unwrap();
    if vec.len() == 0 {
        mark(ast::Expression_::Binary(op, Box::new(e), Box::new(e2)), lo, hi)
    } else {
        mark(ast::Expression_::Binary(
            op, Box::new(e), Box::new(vec_to_expr(e2, vec, lo, hi))), lo, hi)
    }
}
