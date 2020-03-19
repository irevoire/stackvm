/// A variable.
///
/// Parameters
/// ----------
/// name : str
///     The name of the variable.
///
/// Note
/// ----
/// A variable is uniquely indexed by a name.
#[derive(Debug)]
pub struct Var {
    pub name: String,
}

impl Var {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl crate::Graph for Var {
    fn name(&self) -> &str {
        "Var"
    }

    fn graph(&self, index: usize) -> usize {
        let name = index;
        let value = index + 1;
        println!("\t{} [label = {}];", name, self.name());
        println!("\t{} [label = \"{}\"];", value, self.name);
        println!("\t{} -> {};", name, value);
        value
    }
}

use crate::inst;

impl Var {
    pub fn compile(&self) -> Vec<inst::Inst> {
        vec![inst::vload(&self.name)]
    }

    pub fn optimize(&self) -> super::Expr {
        super::var(&self.name)
    }
}
