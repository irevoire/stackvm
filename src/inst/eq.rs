/// Pop top two values from the stack, if they are equal push `1` else push `0`.
///
///  Note
/// ----
/// result = a == b where a = stack[-2], b = stack[-1]
#[derive(Default, Debug)]
pub struct Eq {}

crate::empty_display!(Eq);
crate::empty_new!(Eq);

impl Eq {
    /// execute the instruction and return the offset you should add to the
    /// program counter
    pub fn run(&self, stack: &mut crate::Stack, _vmap: &mut crate::VMap) -> i64 {
        let a = stack.pop().expect("No more value in the stack");
        let b = stack.pop().expect("No more value in the stack");
        stack.push((a == b).into());
        1
    }
}
