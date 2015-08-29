/// Type Checker
///
/// This is a simple typechecker that only verifies simple properties of our
/// program. Right now we only have one type, so there's not really a whole lot
/// of typechecking to do.

use std::collections::HashMap;

use parse::ast::{self, Expr_, Statement_};
use util::{Mark, DUMMY_MARK};

struct TypeChecker<'a> {
    prog: &'a ast::Program,
    syms: HashMap<ast::Ident, bool>,
    return_found: bool,
}

pub fn typecheck(p: &ast::Program) {
    let mut tc = TypeChecker::new(p);
    for stm in p.statements.iter() {
        tc.stm(stm);
    }
    if !tc.return_found {
        p.errors.add(&DUMMY_MARK, "main does not return");
    }
}

impl<'a> TypeChecker<'a> {
    fn new(p: &ast::Program) -> TypeChecker {
        TypeChecker {
            prog: p,
            syms: HashMap::new(),
            return_found: false,
        }
    }

    fn stm(&mut self, s: &ast::Statement) {
        match s.node {
            Statement_::Decl(id) => self.check_decl(id, &s.mark, false),
            Statement_::DeclAssign(id, ref e) => {
                self.check_decl(id, &s.mark, true);
                self.assign(id, &s.mark, e);
            }
            Statement_::Assign(id, ref e) => self.assign(id, &s.mark, e),
            Statement_::Return(ref e) => {
                self.return_found = true;
                self.expr(e)
            }
        }
    }

    fn assign(&mut self, id: ast::Ident, mark: &Mark, e: &ast::Expr) {
        self.expr(e);
        match self.syms.insert(id, true) {
            Some(..) => {}
            None => {
                let msg = format!("undeclared variable `{}`", id);
                self.prog.errors.add(mark, &msg);
            }
        }
    }

    fn expr(&mut self, e: &ast::Expr) {
        match e.node {
            Expr_::Variable(id) => {
                match self.syms.get(&id).map(|b| *b) {
                    Some(false) if !self.return_found => {
                        let msg = format!("uninitialized variable `{}`", id);
                        self.prog.errors.add(&e.mark, &msg);
                    }
                    Some(..) => {}
                    None => {
                        let msg = format!("undeclared variable `{}`", id);
                        self.prog.errors.add(&e.mark, &msg);
                    }
                }
            }
            Expr_::Constant(..) => {}
            Expr_::Unary(_, ref e) => self.expr(e),
            Expr_::Binary(_, ref e1, ref e2) => {
                self.expr(e1);
                self.expr(e2);
            }
        }
    }

    fn check_decl(&mut self, id: ast::Ident, mark: &Mark, initialized: bool) {
        match self.syms.insert(id, initialized) {
            Some(..) => {
                let msg = format!("redeclared variable `{}`", id);
                self.prog.errors.add(mark, &msg);
            }
            None => {}
        }
    }
}
