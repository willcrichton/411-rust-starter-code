//! Assembly language
//!
//! Currently just a pseudo-language with 3-operand instructions and arbitrarily
//! many temps

use std::fmt;

use util::temp;

#[derive(Clone)]
pub enum Instruction {
    Binop(Operation, Operand, Operand, Option<Operand>),
    Mov(Operand, Operand),
    Directive(String),
    Comment(String),
    Label(String),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Operand {
    Imm(u32),
    Reg(Register),
    Temp(temp::Temp),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Register { EAX, EBX, ECX, EDX, R11 }

#[derive(Clone)]
pub enum Operation { Add, Sub, Mul, Div, Mod }

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Instruction::*;
        match *self {
            Label(ref s) => write!(f, "{:?}:", s),
            Binop(ref op, ref d, ref s1, ref s2) => write!(f, "\t{:?}\t{:?} <- {:?},{:?}",
                                                       op, d, s1, s2),
            Mov(ref d, ref s) => write!(f, "\tMOVL\t{:?}, {:?}", s, d),
            Directive(ref s) => write!(f, "\t{:?}", s),
            Comment(ref s) => write!(f, "\t/* {:?} */", s),
        }

    }
}

impl fmt::Debug for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Operand::*;
        match *self {
            Imm(c) => write!(f, "${:?}", c),
            Temp(t) => write!(f, "{:?}", t),
            Reg(ref r) => write!(f, "{:?}", r),
        }
    }
}

impl fmt::Debug for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Register::*;
        match *self {
            EAX => "%eax".fmt(f),
            EBX => "%ebx".fmt(f),
            ECX => "%ecx".fmt(f),
            EDX => "%edx".fmt(f),
            R11 => "%r11".fmt(f),
        }
    }
}

impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Operation::*;
        match *self {
            Add => "ADD".fmt(f),
            Sub => "SUB".fmt(f),
            Mul => "MUL".fmt(f),
            Div => "DIV".fmt(f),
            Mod => "MOD".fmt(f),
        }
    }
}
