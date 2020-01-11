/// pc = pc + offset if the value on the top of the stack is true.
/// otherwise pc = pc + 1
///
/// Parameters
/// ----------
/// offset: int
///    Relative offset in the PC.
#[derive(Debug)]
pub struct CondJump {
    offset: i64,
}

impl CondJump {
    pub fn new(offset: i64) -> Self {
        Self { offset }
    }

    /// execute the instruction and return the offset you should add to the
    /// program counter
    pub fn run(&self, stack: &mut crate::Stack, _vmap: &mut crate::VMap) -> i64 {
        match stack.pop().expect("No value left in the stack") {
            // false
            0 => 1,
            _ => self.offset,
        }
    }
}

impl std::fmt::Display for CondJump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cond_jump {}", self.offset)
    }
}
