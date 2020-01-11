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
    name: String,
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

use crate::inst;

impl Var {
    pub fn compile(&self) -> Vec<inst::Inst> {
        vec![inst::vload(&self.name)]
    }
}
