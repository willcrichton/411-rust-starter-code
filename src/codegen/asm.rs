//! Assembly language
//!
//! Currently just a pseudo-language with 3-operand instructions and arbitrarily
//! many temps

use std::fmt;

use util::Temp;

#[derive(Clone)]
pub enum Instruction {
    Binop(Op, Operand, Operand, Operand),
    Mov(Operand, Operand),
    Directive(String),
    Comment(String),
    Label(String),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Operand {
    Imm(u32),
    Reg(Register),
    Temp(Temp),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Register { EAX }

#[derive(Clone)]
pub enum Op { Add, Sub, Mul, Div, Mod }

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::Label(ref s) => write!(f, "{}:", s),
            Instruction::Binop(ref op, ref d, ref s1, ref s2) =>
                write!(f, "\t{} <-- {} {} {}", d, s1, op, s2),
            Instruction::Mov(ref d, ref s) => write!(f, "\t{} <-- {}", d, s),
            Instruction::Directive(ref s) => write!(f, "\t{}", s),
            Instruction::Comment(ref s) => write!(f, "\t/* {} */", s),
        }

    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operand::Imm(c) => write!(f, "${}", c),
            Operand::Temp(t) => write!(f, "{}", t),
            Operand::Reg(ref r) => write!(f, "{}", r),
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Register::EAX => "%eax".fmt(f),
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Op::Add => "+".fmt(f),
            Op::Sub => "-".fmt(f),
            Op::Mul => "*".fmt(f),
            Op::Div => "/".fmt(f),
            Op::Mod => "%".fmt(f),
        }
    }
}
