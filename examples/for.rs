//! This function provides an statement and
//! the corresponding manually written stackvm instructions.

use stackvm::{expr, inst, stmt};
use std::collections::HashMap;

fn main() {
    let extent = 4;

    let x = &expr::var("x");
    let i = &expr::var("i");
    let stmt = stmt::r#for("i", expr::r#const(extent), stmt::assign("x", x + i));

    println!("{}", stmt);

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
