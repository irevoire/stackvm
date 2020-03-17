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
    var: String,
    extent: Expr,
    body: Stmt,
}

impl For {
    pub fn new(var: &str, extent: Expr, body: Stmt) -> Self {
        Self {
            var: var.to_string(),
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

impl crate::Graph for For {
    fn name(&self) -> &str {
        "For"
    }

    fn graph(&self, index: usize) -> usize {
        let name = index;
        let var = index + 1;
        let extent = index + 2;

        println!("\t{} [label = {}];", name, self.name());
        println!("\t{} [label = {}];", var, self.var);
        println!("\t{} [label = {}];", extent, self.extent.name());

        println!("\t{} -> {};", name, var);
        println!("\t{} -> {};", name, extent);
        let body = self.extent.graph(extent) + 1;

        println!("\t{} [label = {}];", body, self.body.name());
        println!("\t{} -> {};", name, body);
        self.body.graph(body)
    }
}

use crate::{expr, inst};

impl For {
    pub fn compile(&self) -> Vec<inst::Inst> {
        // init self.var to 0
        let mut c = super::assign(&self.var, expr::r#const(0)).compile();
        // we’ll want to come back to this position
        let cmp = c.len() as i64;
        // the loop start: we pull self.var and extent
        c.append(&mut expr::var(&self.var).compile());
        c.append(&mut self.extent.compile());
        // check if they are equals
        c.push(inst::eq());
        // if true: exit the loop => advance of the size of the body

        // we need to compile the body to get its size
        let mut body = self.body.compile();
        // we increment self.name by one
        body.append(&mut (&expr::var("i") + &expr::r#const(1)).compile());

        // +1 is for the final instruction that was not added right now
        let end = cmp + body.len() as i64 + 1;
        c.push(inst::condjump(end as i64));

        c.append(&mut body);

        // we come back to the check we saved before
        // we need to add +1 because we are adding an instruction
        c.push(inst::jump(cmp - (c.len() + 1) as i64));
        c
    }
}
