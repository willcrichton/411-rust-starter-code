//! Abstract Syntax Trees
//!
//! This module contains the AST for the l1c compiler, as well as utilities
//! necessary to print the program.

use std::fmt;

use util::{mark, errors, symbol};
use util::errors::Errors;
use util::symbol::Symbol;

pub struct Program {
    pub statements: Vec<Statement>,
    pub errors: Errors,
}

pub type Statement = mark::Marked<Statement_>;
pub enum Statement_ {
    Decl(Ident),
    DeclAssign(Ident, Expression),
    Assign(Ident, Expression),
    Return(Expression),
}

pub type Expression = mark::Marked<Expression_>;
pub enum Expression_ {
    Variable(Ident),
    Constant(u32),
    Unary(Operator, Box<Expression>),
    Binary(Operator, Box<Expression>, Box<Expression>),
}

pub type Ident = Symbol;

pub enum Operator {
    Plus,
    Minus,
    Times,
    DividedBy,
    Modulo,
    Negative,
    Decrement,
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "int main() {{"));
        for stm in self.statements.iter() {
            try!(writeln!(f, "  {:?}", stm));
        }
        writeln!(f, "}}")
    }
}

impl fmt::Debug for Statement_ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Statement_::*;
        match *self {
            Decl(ref id) => write!(f, "int {:?};", id),
            DeclAssign(ref id, ref expr) => write!(f, "int {:?} = {:?};", id, expr),
            Assign(ref id, ref expr) => write!(f, "{:?} = {:?};", id, expr),
            Return(ref expr) => write!(f, "return {:?};", expr),
        }
    }
}

impl fmt::Debug for Expression_ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Expression_::*;
        match *self {
            Variable(ref id) => write!(f, "{:?}", id),
            Constant(c) => write!(f, "{:?}", c),
            Unary(ref op, ref e) => write!(f, "{:?}({:?})", op, e),
            Binary(ref op, ref e1, ref e2) => write!(f, "({:?} {:?} {:?})", e1, op, e2),
        }
    }
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Operator::*;
        match *self {
            Plus => "+".fmt(f),
            Negative | Minus => "-".fmt(f),
            Times => "*".fmt(f),
            DividedBy => "/".fmt(f),
            Modulo => "%".fmt(f),
            Decrement => "--".fmt(f),
        }
    }
}
