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

    pub fn optimize(&self) -> Expr {
        let a = self.a.optimize();
        let b = self.b.optimize();

        match (a, b) {
            // simplify addition between constants
            (Expr::Const(a), Expr::Const(b)) => super::r#const(a.value + b.value),
            // if your are adding zero to something, ignore the zero
            (other, Expr::Const(c)) | (Expr::Const(c), other) if c.value == 0 => other,
            // always put the constants first
            (Expr::Add(expra), Expr::Add(exprb)) => match Self::merge(&expra, &exprb) {
                (Some(c), a) => super::add(&c, &a),
                (None, a) => a,
            },
            // always put the constants first c + (expr + expr)
            (Expr::Const(c), Expr::Add(add)) | (Expr::Add(add), Expr::Const(c)) => {
                match (&add.a, &add.b) {
                    // c + (c + expr) => c' + expr
                    (Expr::Const(a), b) | (b, Expr::Const(a)) => {
                        super::add(&super::r#const(a.value + c.value), &b)
                    }
                    // there is no other constants: we do nothing
                    (a, b) => super::add(&super::r#const(c.value), &super::add(&a, &b)),
                }
            }
            // always put the constants first expr + (c + expr)
            (expr, Expr::Add(add)) | (Expr::Add(add), expr) => {
                match (&add.a, &add.b) {
                    (c @ Expr::Const(_), b) | (b, c @ Expr::Const(_)) => {
                        super::add(&c, &super::add(&expr, &b))
                    }
                    // there is no constants: we do nothing
                    (a, b) => super::add(&expr, &super::add(&a, &b)),
                }
            }
            // always put the constants first
            (other, c @ Expr::Const(_)) => super::add(&c, &other),
            // no optimisation were found
            (a, b) => super::add(&a, &b),
        }
    }

    /// here we suppose all the constant are already on the left of each add
    fn merge(a: &Add, b: &Add) -> (Option<Expr>, Expr) {
        match (&a.a, &a.b, &b.a, &b.b) {
            (Expr::Const(const_a), expr_a, Expr::Const(const_b), expr_b) => (
                Some(super::r#const(const_a.value + const_b.value)),
                super::add(&expr_a, &expr_b),
            ),
            (c @ Expr::Const(_), expr_a, expr_b, expr_c)
            | (expr_a, expr_b, c @ Expr::Const(_), expr_c) => (
                Some(c.clone()),
                super::add(&expr_a, &super::add(&expr_b, &expr_c)),
            ),
            // here we do do nothing: merge the two parameters back in a add
            (a, b, c, d) => (None, super::add(&super::add(&a, &b), &super::add(&c, &d))),
        }
    }
}
