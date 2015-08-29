//! AST -> IR translator

use std::collections::HashMap;

use middle::ir::{Binop, Statement, Expr};
use parse::ast::{self, Expr_, Operator, Statement_};
use util::{Temp, TempAllocator, Errors};

pub mod ir;

struct Translator {
    temps: TempAllocator,
    syms: HashMap<ast::Ident, Temp>,
    errors: Errors,
}

pub fn translate(p: ast::Program) -> ir::Program {
    let mut translator = Translator::new(p.errors);
    ir::Program {
        statements: p.statements.iter()
                                .filter_map(|stm| translator.stm(stm))
                                .collect(),
        temps: translator.temps,
    }
}

impl Translator {
    fn new(errors: Errors) -> Translator {
        Translator {
            temps: TempAllocator::new(),
            syms: HashMap::new(),
            errors: errors
        }
    }

    fn stm(&mut self, stm: &ast::Statement) -> Option<Statement> {
        match stm.node {
            Statement_::Assign(id, ref e) |
            Statement_::DeclAssign(id, ref e) => {
                Some(Statement::Move(Expr::Temp(self.temp(id)), self.exp(e)))
            }
            Statement_::Return(ref e) => Some(Statement::Return(self.exp(e))),
            Statement_::Decl(_) => None
        }
    }

    fn exp(&mut self, exp: &ast::Expr) -> Expr {
        match exp.node {
            Expr_::Variable(id) => {
                match self.syms.get(&id) {
                    Some(sym) => Expr::Temp(*sym),
                    None => {
                        let msg = format!("attempted to use variable {} \
                                           before initialization", id);
                        self.errors.die(&exp.mark, &msg);
                    }
                }
            },
            Expr_::Constant(c) => Expr::Constant(c),
            Expr_::Unary(Operator::Negative, ref e) => {
                Expr::Binop(Binop::Sub,
                            Box::new(Expr::Constant(0)),
                            Box::new(self.exp(e)))
            }
            Expr_::Binary(op, ref e1, ref e2) =>
                Expr::Binop(self.op(op),
                            Box::new(self.exp(e1)),
                            Box::new(self.exp(e2))),

            Expr_::Unary(_, _) => unimplemented!(),
        }
    }

    fn temp(&mut self, id: ast::Ident) -> Temp {
        let temp = self.temps.gen();
        self.syms.insert(id, temp);
        temp
    }

    fn op(&self, op: Operator) -> Binop {
        match op {
            Operator::Plus => Binop::Add,
            Operator::Minus => Binop::Sub,
            Operator::Times => Binop::Mul,
            Operator::DividedBy => Binop::Div,
            Operator::Modulo => Binop::Mod,
            Operator::Negative => Binop::Sub, // unary to binary!
            Operator::Decrement => unreachable!(),
        }
    }
}
