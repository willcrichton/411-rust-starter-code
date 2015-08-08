//! AST -> IR translator

use std::collections::HashMap;

use parse::ast;
use util::errors::Errors;
use util::temp;
use ir::ilang;

struct Translator {
    temps: temp::Allocator,
    syms: HashMap<ast::Ident, temp::Temp>,
    statements: Vec<ilang::Statement>,
    errors: Errors,
}

pub fn translate(p: ast::Program) -> ilang::Program {
    let mut translator = Translator::new(p.errors);
    ilang::Program {
        statements: p.statements.into_iter()
            .map(|stm| { translator.stm(stm) })
            .filter(|opt| { opt.is_some() })
            .map(|opt| { opt.unwrap() })
            .collect(),
        temps: translator.temps,
    }
}

impl Translator {
    fn new(errors: Errors) -> Translator {
        Translator {
            temps: temp::Allocator::new(),
            syms: HashMap::new(),
            statements: Vec::new(),
            errors: errors
        }
    }

    fn stm(&mut self, stm: ast::Statement) -> Option<ilang::Statement> {
        match stm.node {
            ast::Statement_::Assign(id, e) | ast::Statement_::DeclAssign(id, e) =>
                Some(ilang::Statement::Move(ilang::Expression::Temp(self.temp(id)), self.exp(e))),
            ast::Statement_::Return(e) => Some(ilang::Statement::Return(self.exp(e))),
            ast::Statement_::Decl(_) => None
        }
    }

    fn exp(&mut self, exp: ast::Expression) -> ilang::Expression {
        match exp.node {
            ast::Expression_::Variable(id) => {
                match self.syms.get(&id) {
                    Some(sym) => ilang::Expression::Temp(*sym),
                    None => self.errors.die(exp.mark, format!("attempted to use variable {:?} before initialization", id))
                }
            },
            ast::Expression_::Constant(c) => ilang::Expression::Constant(c),
            ast::Expression_::Unary(ast::Operator::Negative, e) =>
                ilang::Expression::Binop(
                    ilang::Binop::Sub,
                    Box::new(ilang::Expression::Constant(0)),
                    Box::new(self.exp(*e))),
            ast::Expression_::Binary(op, e1, e2) =>
                ilang::Expression::Binop(
                    self.op(op),
                    Box::new(self.exp(*e1)),
                    Box::new(self.exp(*e2))),

            // should be impossible
            ast::Expression_::Unary(_, _) => unimplemented!()
        }
    }

    fn temp(&mut self, id: ast::Ident) -> temp::Temp {
        let temp = self.temps.gen();
        self.syms.insert(id, temp);
        temp
    }

    fn op(&self, op: ast::Operator) -> ilang::Binop {
        match op {
            ast::Operator::Plus => ilang::Binop::Add,
            ast::Operator::Minus => ilang::Binop::Sub,
            ast::Operator::Times => ilang::Binop::Mul,
            ast::Operator::DividedBy => ilang::Binop::Div,
            ast::Operator::Modulo => ilang::Binop::Mod,
            ast::Operator::Negative => ilang::Binop::Sub, // unary to binary!
            ast::Operator::Decrement => unreachable!(),
        }
    }
}
