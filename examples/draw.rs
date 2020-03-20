use stackvm::{expr, Graph};

fn main() {
    let a = &expr::var("a");
    let b = &expr::r#const(1);

    let expr = &(a + b) + &(a + &(a + b));

    println!("digraph {{");
    // expr.graph(0);
    expr.optimize().graph(0);
    println!("}}");
}
