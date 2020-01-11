/// Pop top two values from the stack, push their product.
///
///  Note
/// ----
/// result = a * b where a = stack[-2], b = stack[-1]
#[derive(Default, Debug)]
pub struct Mul {}

crate::empty_display!(Mul);
crate::empty_new!(Mul);

impl Mul {
    /// execute the instruction and return the offset you should add to the
    /// program counter
    pub fn run(&self, stack: &mut crate::Stack, _vmap: &mut crate::VMap) -> i64 {
        let a = stack.pop().expect("No more value in the stack");
        let b = stack.pop().expect("No more value in the stack");
        stack.push(a * b);
        1
    }
}
