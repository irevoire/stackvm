use super::Expr;

/// AST Node of a == b
///
/// Parameters
/// ----------
/// a : Expr
///     The left operand
///
/// b : Expr
///     The right operand
#[derive(Debug)]
pub struct Eq {
    a: Expr,
    b: Expr,
}

impl Eq {
    pub fn new(a: Expr, b: Expr) -> Self {
        Self { a, b }
    }
}

impl std::fmt::Display for Eq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} == {})", self.a, self.b)
    }
}

use crate::inst;

impl Eq {
    pub fn compile(&self) -> Vec<inst::Inst> {
        let mut c = self.a.compile();
        c.append(&mut self.b.compile());
        c.push(inst::eq());
        c
    }
}
