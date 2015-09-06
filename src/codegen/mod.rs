//! Translation from IR to assembly
//!
//! Currently implements a "convenient munch" algorithm

use middle::ir::{Expr, Binop, Statement, Program};
use util::TempAllocator;
use codegen::asm::{Operand, Instruction, Register, Op};

pub mod asm;

struct Translator {
    ins: Vec<Instruction>,
    temps: TempAllocator,
}

pub fn translate(ir: Program) -> Vec<Instruction> {
    let Program { statements, temps } = ir;
    let mut translator = Translator::new(temps);
    for stm in statements.into_iter() {
        translator.stm(stm);
    }
    translator.ins
}

impl Translator {
    fn new(temps: TempAllocator) -> Translator {
        Translator { ins: Vec::new(), temps: temps }
    }

    fn stm(&mut self, s: Statement) {
        match s {
            Statement::Move(Expr::Temp(t), e) =>
                self.exp(Operand::Temp(t), e),
            Statement::Move(..) => unreachable!(),
            // return e is implented as %eax <- e
            Statement::Return(e) => {
                self.exp(Operand::Reg(Register::EAX), e);
            }
        }
    }

    /// Generates instruction to achieve `dst <- e`
    fn exp(&mut self, dst: Operand, e: Expr) {
        let ins = match e {
            Expr::Constant(c) => Instruction::Mov(dst, Operand::Imm(c)),
            Expr::Temp(c) => Instruction::Mov(dst, Operand::Temp(c)),
            Expr::Binop(binop, e1, e2) => {
                let t1 = Operand::Temp(self.temps.gen());
                let t2 = Operand::Temp(self.temps.gen());
                self.exp(t1.clone(), *e1);
                self.exp(t2.clone(), *e2);
                Instruction::Binop(self.op(binop), dst, t1, Some(t2))
            }
        };
        self.ins.push(ins);
    }

    fn op(&self, op: Binop) -> Op {
        match op {
            Binop::Add => Op::Add,
            Binop::Sub => Op::Sub,
            Binop::Mul => Op::Mul,
            Binop::Div => Op::Div,
            Binop::Mod => Op::Mod,
        }
    }
}
