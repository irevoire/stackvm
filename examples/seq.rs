//! This function provides an statement and
//! the corresponding manually written stackvm instructions.

use stackvm::inst;
use std::collections::HashMap;

fn main() {
    /*
    let x = var::Var("x");
    let y = var::Var("y");
    let z = var::Var("z");
    let one = constexpr::ConstExpr(1);
    */

    let program = vec![
        inst::vload("x"),
        inst::vload("y"),
        inst::add(),
        inst::vload("y"),
        inst::mul(),
        inst::vstore("z"),
        inst::vload("z"),
        inst::pushconst(1),
        inst::add(),
        inst::vstore("z"),
    ];

    let mut vmap = HashMap::new();
    vmap.insert("x".to_string(), 2);
    vmap.insert("y".to_string(), 3);

    stackvm::run(&program, &mut vmap, false);
    println!("{:?}", vmap);
}
