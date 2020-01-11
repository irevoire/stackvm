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
