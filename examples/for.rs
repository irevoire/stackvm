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
    let extent = 4;

    let program = vec![
        inst::pushconst(0),
        inst::vstore("i"),
        inst::vload("i"),
        inst::pushconst(extent),
        inst::eq(),
        inst::condjump(10),
        inst::vload("x"),
        inst::vload("i"),
        inst::add(),
        inst::vstore("x"),
        inst::vload("i"),
        inst::pushconst(1),
        inst::add(),
        inst::vstore("i"),
        inst::jump(-12),
    ];

    let mut vmap = HashMap::new();
    vmap.insert("x".to_string(), 1);

    stackvm::run(&program, &mut vmap, false);
    println!("{:?}", vmap);
}
