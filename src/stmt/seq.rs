/// Execute sequence of statements in order.
///
/// Parameters
/// ----------
/// seq_list : List[Stmt]
#[derive(Debug)]
pub struct Seq {
    list: Vec<super::Stmt>,
}

impl Seq {
    pub fn new(list: Vec<super::Stmt>) -> Self {
        Self { list }
    }
}

impl std::fmt::Display for Seq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Seq {{")?;
        for stmt in self.list.iter() {
            writeln!(f, "\t{}", stmt)?;
        }
        writeln!(f, "}}")
    }
}

use crate::inst;

impl Seq {
    pub fn compile(&self) -> Vec<inst::Inst> {
        self.list.iter().flat_map(|stmt| stmt.compile()).collect()
    }
}
