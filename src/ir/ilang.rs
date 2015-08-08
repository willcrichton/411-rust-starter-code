//! IR tree

use std::fmt;
use util::temp;

pub struct Program {
    pub statements: Vec<Statement>,
    pub temps: temp::Allocator,
}

pub enum Statement {
    Move(Expression, Expression),
    Return(Expression),
}

pub enum Expression {
    Constant(u32),
    Temp(temp::Temp),
    Binop(Binop, Box<Expression>, Box<Expression>),
}

pub enum Binop { Add, Sub, Mul, Div, Mod }

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, statement) in self.statements.iter().enumerate() {
            if i > 0 { try!(write!(f, "\n")) }
            try!(write!(f, "{:?}", statement))
        }
        Ok(())
    }
}

impl fmt::Debug for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Statement::*;
        match *self {
            Move(ref e1, ref e2) => write!(f, "{:?} <-- {:?}", e1, e2),
            Return(ref e) => write!(f, "return {:?}", e),
        }
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Expression::*;
        match *self {
            Constant(c) => write!(f, "{:?}", c),
            Temp(ref t) => write!(f, "{:?}", t),
            Binop(ref b, ref e1, ref e2) => write!(f, "({:?} {:?} {:?})", e1, b, e2),
        }
    }
}

impl fmt::Debug for Binop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Binop::*;
        match *self {
            Add => "+".fmt(f),
            Sub => "-".fmt(f),
            Mul => "*".fmt(f),
            Div => "/".fmt(f),
            Mod => "%".fmt(f),
        }
    }
}
