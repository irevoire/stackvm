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

use crate::inst;

impl crate::Graph for Assign {
    fn name(&self) -> &str {
        "Assign"
    }

    fn graph(&self, index: usize) -> usize {
        let name = index;
        let ident = index + 1;
        let expr = index + 2;

        println!("\t{} [label = {}];", name, self.name());
        println!("\t{} [label = {}];", ident, self.name);
        println!("\t{} [label = {}];", expr, self.expr.name());

        println!("\t{} -> {};", name, ident);
        println!("\t{} -> {};", name, expr);
        self.expr.graph(index + 2)
    }
}

impl Assign {
    pub fn compile(&self) -> Vec<inst::Inst> {
        let mut base = self.expr.compile();
        base.push(inst::vstore(&self.name));
        base
    }
}
