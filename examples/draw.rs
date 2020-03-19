use stackvm::{expr, Graph};

fn main() {
    let a = &expr::r#const(1);
    let b = &expr::r#const(1);

    let expr = a + b;

    println!("digraph {{");
    expr.optimize().graph(0);
    println!("}}");
}
