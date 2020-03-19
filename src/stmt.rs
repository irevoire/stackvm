mod assign;
mod r#for;
mod seq;

use crate::expr::Expr;

pub fn seq(list: Vec<Stmt>) -> Stmt {
    Stmt::Seq(Box::new(seq::Seq::new(list)))
}
pub fn r#for(var: &str, extent: Expr, body: Stmt) -> Stmt {
    Stmt::For(Box::new(r#for::For::new(var, extent, body)))
}

pub fn assign(name: &str, expr: crate::expr::Expr) -> Stmt {
    Stmt::Assign(Box::new(assign::Assign::new(name, expr)))
}

/// Base enum for all statements.
#[derive(Debug)]
pub enum Stmt {
    Seq(Box<seq::Seq>),
    For(Box<r#for::For>),
    Assign(Box<assign::Assign>),
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Seq(a) => a.fmt(f),
            Stmt::For(a) => a.fmt(f),
            Stmt::Assign(a) => a.fmt(f),
        }
    }
}

impl crate::Graph for Stmt {
    fn name(&self) -> &str {
        "Stmt"
    }

    fn graph(&self, index: usize) -> usize {
        match self {
            Stmt::Seq(inner) => inner.graph(index),
            Stmt::For(inner) => inner.graph(index),
            Stmt::Assign(inner) => inner.graph(index),
        }
    }
}

impl Stmt {
    pub fn compile(&self) -> Vec<crate::inst::Inst> {
        match self {
            Stmt::Seq(a) => a.compile(),
            Stmt::For(a) => a.compile(),
            Stmt::Assign(a) => a.compile(),
        }
    }

    pub fn optimize(&self) -> Stmt {
        match self {
            Stmt::Seq(inner) => inner.optimize(),
            Stmt::For(inner) => inner.optimize(),
            Stmt::Assign(inner) => inner.optimize(),
        }
    }
}
