/// Pop the top of the stack and store to vmap.
///
/// Parameters
/// ----------
/// name : str
///    The variable name.
#[derive(Debug)]
pub struct VStore {
    name: String,
}

impl VStore {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    /// execute the instruction and return the offset you should add to the
    /// program counter
    pub fn run(&self, stack: &mut crate::Stack, vmap: &mut crate::VMap) -> i64 {
        let value = stack.pop().expect("No more value in the stack");
        vmap.insert(self.name.clone(), value);
        1
    }
}

impl std::fmt::Display for VStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vstore {}", self.name)
    }
}
