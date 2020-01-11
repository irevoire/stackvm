//! Instructions of a stackvm

mod add;
mod condjump;
mod eq;
mod jump;
mod macros;
mod mul;
mod pushconst;
mod vload;
mod vstore;

pub fn add() -> Inst {
    Inst::Add(add::Add::new())
}

pub fn mul() -> Inst {
    Inst::Mul(mul::Mul::new())
}

pub fn eq() -> Inst {
    Inst::Eq(eq::Eq::new())
}

pub fn condjump(offset: i64) -> Inst {
    Inst::CondJump(condjump::CondJump::new(offset))
}

pub fn jump(offset: i64) -> Inst {
    Inst::Jump(jump::Jump::new(offset))
}

pub fn pushconst(val: i64) -> Inst {
    Inst::PushConst(pushconst::PushConst::new(val))
}

pub fn vload(name: &str) -> Inst {
    Inst::VLoad(vload::VLoad::new(name))
}

pub fn vstore(name: &str) -> Inst {
    Inst::VStore(vstore::VStore::new(name))
}

/// Base enum for all StackVM instructions.
#[derive(Debug)]
pub enum Inst {
    Add(add::Add),
    Mul(mul::Mul),
    Eq(eq::Eq),
    VLoad(vload::VLoad),
    VStore(vstore::VStore),
    PushConst(pushconst::PushConst),
    CondJump(condjump::CondJump),
    Jump(jump::Jump),
}

impl Inst {
    pub fn run(&self, stack: &mut crate::Stack, vmap: &mut crate::VMap) -> i64 {
        match self {
            Inst::Add(a) => a.run(stack, vmap),
            Inst::Mul(a) => a.run(stack, vmap),
            Inst::Eq(a) => a.run(stack, vmap),
            Inst::VLoad(a) => a.run(stack, vmap),
            Inst::VStore(a) => a.run(stack, vmap),
            Inst::PushConst(a) => a.run(stack, vmap),
            Inst::CondJump(a) => a.run(stack, vmap),
            Inst::Jump(a) => a.run(stack, vmap),
        }
    }
}

impl std::fmt::Display for Inst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Inst::Add(a) => a.fmt(f),
            Inst::Mul(a) => a.fmt(f),
            Inst::Eq(a) => a.fmt(f),
            Inst::VLoad(a) => a.fmt(f),
            Inst::VStore(a) => a.fmt(f),
            Inst::PushConst(a) => a.fmt(f),
            Inst::CondJump(a) => a.fmt(f),
            Inst::Jump(a) => a.fmt(f),
        }
    }
}
