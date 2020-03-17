use stackvm::Graph;
use stackvm::{expr, stmt};

fn main() {
    let extent = 4;

    let x = &expr::var("x");
    let i = &expr::var("i");
    let stmt = stmt::r#for("i", expr::r#const(extent), stmt::assign("x", x + i));

    println!("digraph {{");
    stmt.graph(0);
    println!("}}");
}
