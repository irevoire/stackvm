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

impl crate::Graph for Eq {
    fn name(&self) -> &str {
        "Eq"
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

impl Eq {
    pub fn compile(&self) -> Vec<inst::Inst> {
        let mut c = self.a.compile();
        c.append(&mut self.b.compile());
        c.push(inst::eq());
        c
    }

    pub fn optimize(&self) -> Expr {
        let a = self.a.optimize();
        let b = self.b.optimize();
        match (a, b) {
            (Expr::Const(a), Expr::Const(b)) if a.value == b.value => super::r#const(1),
            (Expr::Const(_), Expr::Const(_)) => super::r#const(0),
            (Expr::Var(a), Expr::Var(b)) if a.name == b.name => super::r#const(1),
            (a, b) => super::eq(&a, &b),
        }
    }
}
