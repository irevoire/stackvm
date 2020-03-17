mod assign;
mod r#for;
mod seq;

use crate::expr::Expr;
use std::rc::Rc;

pub fn seq(list: Vec<Stmt>) -> Stmt {
    Stmt::Seq(Rc::new(seq::Seq::new(list)))
}
pub fn r#for(var: &str, extent: Expr, body: Stmt) -> Stmt {
    Stmt::For(Rc::new(r#for::For::new(var, extent, body)))
}

pub fn assign(name: &str, expr: crate::expr::Expr) -> Stmt {
    Stmt::Assign(Rc::new(assign::Assign::new(name, expr)))
}

/// Base enum for all statements.
#[derive(Debug)]
pub enum Stmt {
    Seq(Rc<seq::Seq>),
    For(Rc<r#for::For>),
    Assign(Rc<assign::Assign>),
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
        self.inner().graph(index)
    }
}

impl Stmt {
    fn inner(&self) -> Rc<dyn crate::Graph> {
        match self {
            Stmt::Seq(inner) => inner.clone(),
            Stmt::For(inner) => inner.clone(),
            Stmt::Assign(inner) => inner.clone(),
        }
    }

    pub fn compile(&self) -> Vec<crate::inst::Inst> {
        match self {
            Stmt::Seq(a) => a.compile(),
            Stmt::For(a) => a.compile(),
            Stmt::Assign(a) => a.compile(),
        }
    }
}
