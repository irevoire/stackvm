/// pc = pc + offset
///
/// Parameters
/// ----------
/// offset: int
///     Relative offset in the PC.
#[derive(Debug)]
pub struct Jump {
    offset: i64,
}

impl Jump {
    pub fn new(offset: i64) -> Self {
        Self { offset }
    }

    /// execute the instruction and return the offset you should add to the
    /// program counter
    pub fn run(&self, _stack: &mut crate::Stack, __vmap: &mut crate::VMap) -> i64 {
        self.offset
    }
}

impl std::fmt::Display for Jump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "jump {}", self.offset)
    }
}
