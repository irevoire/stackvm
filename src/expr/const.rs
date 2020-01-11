/// A constant value.
///
/// Parameters
/// ----------
/// value : int
///     The value.
#[derive(Debug)]
pub struct Const {
    value: i64,
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
