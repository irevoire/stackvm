use super::Stmt;
use crate::expr::Expr;

/// A for statement
///
/// for (loop_var = 0; loop_var < extent; ++loop_var) {
///    body
/// }
///
/// Parameters
/// ----------
/// loop_var : str
///     The loop variable.
///
/// Parameters
/// ----------
/// loop_var : str
///     The loop variable.
///
/// begin : Expr
///     The beginning of the loop variable.
///
/// end : Expr
///     The end of the loop,
#[derive(Debug)]
pub struct For {
    var: Expr,
    extent: Expr,
    body: Stmt,
}

impl For {
    pub fn new(var: &str, extent: Expr, body: Stmt) -> Self {
        Self {
            var: crate::expr::var(var),
            extent,
            body,
        }
    }
}

impl std::fmt::Display for For {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "For {} := 0 to {} {{", self.var, self.extent)?;
        writeln!(f, "\t{}", self.body)?;
        writeln!(f, "}}")
    }
}
