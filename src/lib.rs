pub mod expr;
pub mod inst;
pub mod stmt;

use std::collections::HashMap;

type Stack = Vec<i64>;
type VMap = HashMap<String, i64>;

pub trait Graph {
    /// return the name of the element
    fn name(&self) -> &str;

    /// generate a graph
    /// need to know what was the number of the last element in the graph
    fn graph(&self, index: usize) -> usize;
}

pub fn run(program: &[inst::Inst], vmap: &mut VMap, debug_print: bool) {
    let mut pc = 0_i64;
    let mut stack = Vec::new();

    while (pc as usize) < program.len() {
        let inst = &program[pc as usize]; // TODO this can crash
        if debug_print {
            println!("pc={}\tinst={}\tvmap={:?}", pc, inst, vmap);
        }
        pc += inst.run(&mut stack, vmap);
    }
}
