//! IR, or intermediate representation

use std::fmt;

use util::{Temp, TempAllocator};

pub struct Program {
    pub statements: Vec<Statement>,
    pub temps: TempAllocator,
}

pub enum Statement {
    Move(Expr, Expr),
    Return(Expr),
}

pub enum Expr {
    Constant(u32),
    Temp(Temp),
    Binop(Binop, Box<Expr>, Box<Expr>),
}

pub enum Binop { Add, Sub, Mul, Div, Mod }

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, statement) in self.statements.iter().enumerate() {
            if i > 0 { try!(write!(f, "\n")) }
            try!(write!(f, "{}", statement))
        }
        Ok(())
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Statement::Move(ref e1, ref e2) => write!(f, "{} <-- {}", e1, e2),
            Statement::Return(ref e) => write!(f, "return {}", e),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::Constant(c) => write!(f, "{}", c),
            Expr::Temp(ref t) => write!(f, "{}", t),
            Expr::Binop(ref b, ref e1, ref e2) => {
                write!(f, "({} {} {})", e1, b, e2)
            }
        }
    }
}

impl fmt::Display for Binop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Binop::Add => "+".fmt(f),
            Binop::Sub => "-".fmt(f),
            Binop::Mul => "*".fmt(f),
            Binop::Div => "/".fmt(f),
            Binop::Mod => "%".fmt(f),
        }
    }
}

