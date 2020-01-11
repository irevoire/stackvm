/// Load variable from vmap to the top of the stack.
///
/// Parameters
/// ----------
/// name : str
///    The variable name.
#[derive(Debug)]
pub struct VLoad {
    name: String,
}

impl VLoad {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    /// execute the instruction and return the offset you should add to the
    /// program counter
    pub fn run(&self, stack: &mut crate::Stack, vmap: &mut crate::VMap) -> i64 {
        stack.push(*vmap.get(&self.name).expect(&format!(
            "You tried to access the unknown var {}",
            self.name
        )));
        1
    }
}

impl std::fmt::Display for VLoad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vload {}", self.name)
    }
}
