/// Type Checker
///
/// This is a simple typechecker that only verifies simple properties of our
/// program. Right now we only have one type, so there's not really a whole lot
/// of typechecking to do.

use std::collections::HashMap;

use parse::ast;
use util::mark;

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
        p.errors.add(&mark::DUMMY, String::from("main does not return"));
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
            ast::Statement_::Decl(id) => self.check_decl(id, &s.mark, false),
            ast::Statement_::DeclAssign(id, ref e) => {
                self.check_decl(id, &s.mark, true);
                self.assign(id, &s.mark, e);
            }
            ast::Statement_::Assign(id, ref e) => self.assign(id, &s.mark, e),
            ast::Statement_::Return(ref e) => { self.return_found = true; self.exp(e) }
        }
    }

    fn assign(&mut self, id: ast::Ident, mark: &mark::Mark, e: &ast::Expression) {
        self.exp(e);
        match self.syms.insert(id, true) {
            Some(..) => {}
            None => {
                let msg = format!("undeclared variable `{}`", id);
                self.prog.errors.add(mark, msg);
            }
        }
    }

    fn exp(&mut self, e: &ast::Expression) {
        match e.node {
            ast::Expression_::Variable(id) => {
                match self.syms.get(&id).map(|b| *b) {
                    Some(false) if !self.return_found => {
                        let msg = format!("uninitialized variable `{}`", id);
                        self.prog.errors.add(&e.mark, msg);
                    }
                    Some(..) => {}
                    None => {
                        let msg = format!("undeclared variable `{}`", id);
                        self.prog.errors.add(&e.mark, msg);
                    }
                }
            }
            ast::Expression_::Constant(..) => {}
            ast::Expression_::Unary(_, ref e) => self.exp(&**e),
            ast::Expression_::Binary(_, ref e1, ref e2) => { self.exp(&**e1); self.exp(&**e2); }
        }
    }

    fn check_decl(&mut self, id: ast::Ident, mark: &mark::Mark, initialized: bool) {
        match self.syms.insert(id, initialized) {
            Some(..) => {
                let msg = format!("redeclared variable `{}`", id);
                self.prog.errors.add(mark, msg);
            }
            None => {}
        }
    }
}
