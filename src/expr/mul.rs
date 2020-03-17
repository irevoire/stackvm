use super::Expr;

/// AST Node of a * b
///
/// Parameters
/// ----------
/// a : Expr
///     The left operand
///
/// b : Expr
///     The right operand
#[derive(Debug)]
pub struct Mul {
    a: Box<Expr>,
    b: Box<Expr>,
}

impl Mul {
    pub fn new(a: Expr, b: Expr) -> Self {
        Self {
            a: Box::new(a),
            b: Box::new(b),
        }
    }
}

impl std::fmt::Display for Mul {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} * {})", self.a, self.b)
    }
}

impl crate::Graph for Mul {
    fn name(&self) -> &str {
        "Mul"
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

impl Mul {
    pub fn compile(&self) -> Vec<inst::Inst> {
        let mut a = self.a.compile();
        a.append(&mut self.b.compile());
        a.push(inst::mul());
        a
    }
}
