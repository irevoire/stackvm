/// A constant value.
///
/// Parameters
/// ----------
/// value : int
///     The value.
#[derive(Debug)]
pub struct Const {
    pub value: i64,
}

impl Const {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

impl std::fmt::Display for Const {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl crate::Graph for Const {
    fn name(&self) -> &str {
        "Const"
    }

    fn graph(&self, index: usize) -> usize {
        let name = index;
        let value = index + 1;
        println!("\t{} [label = {}];", name, self.name());
        println!("\t{} [label = \"{}\"];", value, self.value);
        println!("\t{} -> {};", name, value);
        value
    }
}

use crate::inst;

impl Const {
    pub fn compile(&self) -> Vec<inst::Inst> {
        vec![inst::pushconst(self.value)]
    }

    pub fn optimize(&self) -> super::Expr {
        super::r#const(self.value)
    }
}
