//! Abstract Syntax Trees
//!
//! This module contains the AST for the l1c compiler, as well as utilities
//! necessary to print the program.

use std::fmt;

use util::{Marked, Errors, Symbol};

pub struct Program {
    pub statements: Vec<Statement>,
    pub errors: Errors,
}

pub type Statement = Marked<Statement_>;
#[derive(Clone)]
pub enum Statement_ {
    Decl(Ident),
    DeclAssign(Ident, Expr),
    Assign(Ident, Expr),
    Return(Expr),
}

pub type Expr = Marked<Expr_>;
#[derive(Clone)]
pub enum Expr_ {
    Variable(Ident),
    Constant(u32),
    Unary(Operator, Box<Expr>),
    Binary(Operator, Box<Expr>, Box<Expr>),
}

pub type Ident = Symbol;

#[derive(Copy, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    DividedBy,
    Modulo,
    Negative,
    Decrement,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "int main() {{"));
        for stm in self.statements.iter() {
            try!(writeln!(f, "  {}", stm));
        }
        writeln!(f, "}}")
    }
}

impl fmt::Display for Statement_ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Statement_::Decl(ref id) => write!(f, "int {};", id),
            Statement_::DeclAssign(ref id, ref expr) => {
                write!(f, "int {} = {};", id, expr)
            }
            Statement_::Assign(ref id, ref expr) => {
                write!(f, "{} = {};", id, expr)
            }
            Statement_::Return(ref expr) => write!(f, "return {};", expr),
        }
    }
}

impl fmt::Display for Expr_ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr_::Variable(ref id) => write!(f, "{}", id),
            Expr_::Constant(c) => write!(f, "{}", c),
            Expr_::Unary(ref op, ref e) => write!(f, "{}({})", op, e),
            Expr_::Binary(ref op, ref e1, ref e2) => {
                write!(f, "({} {} {})", e1, op, e2)
            }
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operator::Plus => "+".fmt(f),
            Operator::Negative |
            Operator::Minus => "-".fmt(f),
            Operator::Times => "*".fmt(f),
            Operator::DividedBy => "/".fmt(f),
            Operator::Modulo => "%".fmt(f),
            Operator::Decrement => "--".fmt(f),
        }
    }
}
