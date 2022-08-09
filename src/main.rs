mod node;
use crate::node::{add, create_input, mul, pow_f32, sin};

fn main() {
    // x1, x2, x3, x4 are input nodes of the computational graph:
    let x1 = create_input(1f32);
    let x2 = create_input(2f32);
    let x3 = create_input(3f32);
    let x4 = create_input(3f32);

    // graph variable is the output node of the graph:
    let graph = add(
        x1.clone(),
        mul(
            x2.clone(),
            sin(add(x2.clone(), pow_f32(x3.clone(), x4.clone()))),
        ),
    );

    println!("\nFirst pass, x1 = 1, x2 = 2, x3 = 3");
    let mut result = graph.compute();
    result = round(result, 5);
    println!("Graph output = {}", result);
    assert_eq!(round(result, 5), -0.32727);

    println!("\nSecond pass, same variables");
    let mut result = graph.compute();
    result = round(result, 5);
    println!("Graph output = {}", result);
    assert_eq!(round(result, 5), -0.32727);

    println!("\nThird pass, x1 changed to 2");
    x1.set(2f32);
    let mut result = graph.compute();
    result = round(result, 5);
    println!("Graph output = {}", result);
    assert_eq!(round(result, 5), 0.67273);

    println!("\nFourth pass, x1 = 2, x2 = 3, x3 = 4");
    x1.set(2f32);
    x2.set(3f32);
    x3.set(4f32);
    result = graph.compute();
    result = round(result, 5);
    println!("Graph output = {}", result);
    assert_eq!(round(result, 5), -0.56656);
}

// round to decimal digits
fn round(x: f32, precision: u32) -> f32 {
    let m = 10i32.pow(precision) as f32;
    (x * m).round() / m
}
