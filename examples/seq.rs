//! This function provides an statement and
//! the corresponding manually written stackvm instructions.

use stackvm::{expr, inst, stmt};
use std::collections::HashMap;

fn main() {
    let x = &expr::var("x");
    let y = &expr::var("y");
    let z = &expr::var("z");
    let one = &expr::r#const(1);
    let stmt = stmt::seq(vec![
        stmt::assign("z", &(x + y) * y),
        stmt::assign("z", z + one),
    ]);

    println!("{}", stmt);
    let program = stmt.compile();

    let mut vmap = HashMap::new();
    vmap.insert("x".to_string(), 2);
    vmap.insert("y".to_string(), 3);

    stackvm::run(&program, &mut vmap, false);
    println!("compiled version:\n{:?}", vmap);

    let program2 = vec![
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

    stackvm::run(&program2, &mut vmap, false);
    println!("handwritten version:\n{:?}", vmap);
}
