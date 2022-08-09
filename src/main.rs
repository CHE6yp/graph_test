mod node;
use crate::node::{add, create_input, div, mul, pow_f32, sin, sub};

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

    println!("x1 + x2 * sin(x2 + pow(x3, 3))");
    println!("First pass, x1 = 1, x2 = 2, x3 = 3");
    let mut result = graph.compute();
    result = round(result, 5);
    println!("Graph output = {}\n", result);
    assert_eq!(round(result, 5), -0.32727);

    println!("Second pass, same variables");
    let mut result = graph.compute();
    result = round(result, 5);
    println!("Graph output = {}\n", result);
    assert_eq!(round(result, 5), -0.32727);

    println!("Third pass, x1 changed to 2");
    x1.set(2f32);
    let mut result = graph.compute();
    result = round(result, 5);
    println!("Graph output = {}\n", result);
    assert_eq!(round(result, 5), 0.67273);

    println!("Fourth pass, x1 = 2, x2 = 3, x3 = 4");
    x1.set(2f32);
    x2.set(3f32);
    x3.set(4f32);
    result = graph.compute();
    result = round(result, 5);
    println!("Graph output = {}\n", result);
    assert_eq!(round(result, 5), -0.56656);

    println!("(y1 - y2) / y3");
    println!("y1 = 3, y2 = 2, y3 = 1");
    let y1 = create_input(3f32);
    let y2 = create_input(2f32);
    let y3 = create_input(1f32);
    let graph = div(sub(y1.clone(), y2.clone()), y3.clone());
    result = graph.compute();
    result = round(result, 5);
    println!("Graph output = {}\n", result);
    assert_eq!(round(result, 5), 1f32);

    println!("y1 = 3, y2 = 2, y3 = 0");
    y3.set(0f32);
    result = graph.compute();
    result = round(result, 5);
    println!("Graph output = {}\n", result);
    assert_eq!(result, f32::INFINITY);

    println!("y1 = 3, y2 = 3, y3 = 0");
    y2.set(3f32);
    result = graph.compute();
    result = round(result, 5);
    println!("Graph output = {}\n", result);
    assert_eq!(result.is_nan(), true);
}

// round to decimal digits
fn round(x: f32, precision: u32) -> f32 {
    let m = 10i32.pow(precision) as f32;
    (x * m).round() / m
}
