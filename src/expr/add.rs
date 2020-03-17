use super::Expr;

/// AST Node of a + b
///
/// Parameters
/// ----------
/// a : Expr
///     The left operand
///
/// b : Expr
///     The right operand
#[derive(Debug)]
pub struct Add {
    a: Expr,
    b: Expr,
}

impl Add {
    pub fn new(a: Expr, b: Expr) -> Self {
        Self { a, b }
    }
}

impl std::fmt::Display for Add {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} + {})", self.a, self.b)
    }
}

impl crate::Graph for Add {
    fn name(&self) -> &str {
        "Add"
    }

    fn graph(&self, index: usize) -> usize {
        let name = index;
        let a = index + 1;

        println!("\t{} [label = {}];", name, self.name());
        println!("\t{} [label = {}];", a, self.a.name());
        println!("\t{} -> {};", name, a);

        let b = self.a.graph(a) + 1;

        println!("\t{} [label = {}];", b, self.b.name());
        println!("\t{} -> {};", name, b);
        self.b.graph(b)
    }
}

use crate::inst;

impl Add {
    pub fn compile(&self) -> Vec<inst::Inst> {
        let mut a = self.a.compile();
        a.append(&mut self.b.compile());
        a.push(inst::add());
        a
    }
}
