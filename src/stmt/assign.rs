use crate::expr::Expr;

/// Asssign statement var = expr
///
/// Parameters
/// ----------
/// var_name : str
///     The name of the variable to be assigned.
///
/// expr : Expr
///     The expression to be assigned
#[derive(Debug)]
pub struct Assign {
    name: String,
    expr: Expr,
}

impl Assign {
    pub fn new(name: &str, expr: Expr) -> Self {
        Assign {
            name: name.to_string(),
            expr,
        }
    }
}

impl std::fmt::Display for Assign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} := {}", self.name, self.expr)
    }
}
