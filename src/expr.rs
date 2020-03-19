//! High level program AST nodes.

mod add;
mod r#const;
mod eq;
mod mul;
mod var;

use std::rc::Rc;

pub fn r#const(value: i64) -> Expr {
    Expr::Const(Rc::new(r#const::Const::new(value)))
}

pub fn add(a: &Expr, b: &Expr) -> Expr {
    Expr::Add(Rc::new(add::Add::new(a.clone(), b.clone())))
}

pub fn mul(a: &Expr, b: &Expr) -> Expr {
    Expr::Mul(Rc::new(mul::Mul::new(a.clone(), b.clone())))
}

pub fn eq(a: &Expr, b: &Expr) -> Expr {
    Expr::Eq(Rc::new(eq::Eq::new(a.clone(), b.clone())))
}

pub fn var(name: &str) -> Expr {
    Expr::Var(Rc::new(var::Var::new(name)))
}

/// Base enum for all expressions.
///
/// We put the expression behind Rc so we can clone instead
/// of consuming them when we wand to call `Add` or `Mul`
#[derive(Debug, Clone)]
pub enum Expr {
    Var(Rc<var::Var>),
    Add(Rc<add::Add>),
    Mul(Rc<mul::Mul>),
    Eq(Rc<eq::Eq>),
    Const(Rc<r#const::Const>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Var(a) => a.fmt(f),
            Expr::Add(a) => a.fmt(f),
            Expr::Mul(a) => a.fmt(f),
            Expr::Eq(a) => a.fmt(f),
            Expr::Const(a) => a.fmt(f),
        }
    }
}

impl crate::Graph for Expr {
    fn name(&self) -> &str {
        "Expr"
    }

    fn graph(&self, index: usize) -> usize {
        match self {
            Expr::Var(inner) => inner.graph(index),
            Expr::Add(inner) => inner.graph(index),
            Expr::Mul(inner) => inner.graph(index),
            Expr::Eq(inner) => inner.graph(index),
            Expr::Const(inner) => inner.graph(index),
        }
    }
}

// here, because of lifetime if we add two expression they will be consumed and impossible to
// reuse:
// ```
// (a + b) * a
// ~^~~~~~~~~^ a reused here
// ```
// So we implement the `Add` and `Mul` type only for reference
// of the type. The inconvenient is that, we now need to
// do the `+` and `*` on reference of expression:
// ```
// &(&a + &b) * &a
// ```

impl std::ops::Add for &Expr {
    type Output = Expr;

    fn add(self, other: Self) -> Self::Output {
        add(&self, &other)
    }
}

impl std::ops::Mul for &Expr {
    type Output = Expr;

    fn mul(self, other: Self) -> Self::Output {
        mul(&self, &other)
    }
}

impl Expr {
    pub fn compile(&self) -> Vec<crate::inst::Inst> {
        match self {
            Expr::Var(a) => a.compile(),
            Expr::Add(a) => a.compile(),
            Expr::Mul(a) => a.compile(),
            Expr::Eq(a) => a.compile(),
            Expr::Const(a) => a.compile(),
        }
    }

    pub fn optimize(&self) -> Self {
        match self {
            Expr::Var(inner) => inner.optimize(),
            Expr::Add(inner) => inner.optimize(),
            Expr::Mul(inner) => inner.optimize(),
            Expr::Eq(inner) => inner.optimize(),
            Expr::Const(inner) => inner.optimize(),
        }
    }
}
