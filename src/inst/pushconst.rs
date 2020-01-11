/// Push const value to the top of the stack.
///
/// Parameters
/// ----------
/// value : int
///     The variable name.
///
/// TODO: We should use an Enum over all the primitive type instead of forcing to push
/// an integer
#[derive(Debug)]
pub struct PushConst {
    value: i64,
}

impl PushConst {
    pub fn new(value: i64) -> Self {
        Self { value }
    }

    /// execute the instruction and return the offset you should add to the
    /// program counter
    pub fn run(&self, stack: &mut crate::Stack, _vmap: &mut crate::VMap) -> i64 {
        stack.push(self.value);
        1
    }
}

impl std::fmt::Display for PushConst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pushconst {}", self.value)
    }
}
