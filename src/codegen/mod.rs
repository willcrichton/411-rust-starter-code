//! Translation from IR to assembly
//!
//! Currently implements a "convenient munch" algorithm

use ir::ilang;
use util::temp;

#[allow(dead_code)]
pub mod asm;

struct Translator {
    ins: Vec<asm::Instruction>,
    temps: temp::Allocator,
}

pub fn translate(ir: ilang::Program) -> Vec<asm::Instruction> {
    let ilang::Program { statements, temps } = ir;
    let mut translator = Translator::new(temps);
    translator.ins.push(asm::Instruction::Directive(".globl __c0_main".to_string()));
    translator.ins.push(asm::Instruction::Label("__c0_main".to_string()));
    for stm in statements.into_iter() {
        translator.stm(stm);
    }
    translator.ins
}

impl Translator {
    fn new(temps: temp::Allocator) -> Translator {
        Translator { ins: Vec::new(), temps: temps }
    }

    fn stm(&mut self, s: ilang::Statement) {
        match s {
            ilang::Statement::Move(ilang::Expression::Temp(t), e) =>
                self.exp(asm::Operand::Temp(t), e),
            ilang::Statement::Move(..) => unreachable!(),
            // return e is implented as %eax <- e
            ilang::Statement::Return(e) => {
                self.exp(asm::Operand::Reg(asm::Register::EAX), e);
                self.ins.push(asm::Instruction::Directive("ret".to_string()))
            }
        }
    }

    /// Generates instruction to achieve `dst <- e`
    fn exp(&mut self, dst: asm::Operand, e: ilang::Expression) {
        let ins = match e {
            ilang::Expression::Constant(c) => asm::Instruction::Mov(dst, asm::Operand::Imm(c)),
            ilang::Expression::Temp(c) => asm::Instruction::Mov(dst, asm::Operand::Temp(c)),
            ilang::Expression::Binop(binop, e1, e2) => {
                let t1 = asm::Operand::Temp(self.temps.gen());
                let t2 = asm::Operand::Temp(self.temps.gen());
                self.exp(t1.clone(), *e1);
                self.exp(t2.clone(), *e2);
                asm::Instruction::Binop(self.op(binop), dst, t1, Some(t2))
            }
        };
        self.ins.push(ins);
    }

    fn op(&self, op: ilang::Binop) -> asm::Operation {
        match op {
            ilang::Binop::Add => asm::Operation::Add,
            ilang::Binop::Sub => asm::Operation::Sub,
            ilang::Binop::Mul => asm::Operation::Mul,
            ilang::Binop::Div => asm::Operation::Div,
            ilang::Binop::Mod => asm::Operation::Mod,
        }
    }
}
