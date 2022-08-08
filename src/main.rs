// round to decimal digits
fn round(x: f32, precision: u32) -> f32 {
    let m = 10i32.pow(precision) as f32;
    (x * m).round() / m
}
fn main() {
    // x1, x2, x3 are input nodes of the computational graph:
    // let x1 = create_input("x1");
    // let x2 = create_input("x2");
    // let x3 = create_input("x3");
    let x1 = create_input(1f32);
    let x2 = create_input(2f32);
    let x3 = create_input(3f32);
    // graph variable is the output node of the graph:
    // let graph = add(
    //     x1.clone(),
    //     mul(
    //         x2.clone(),
    //         sin(add(x2.clone(), pow_f32(x3.clone(), Node::Variable(3f32)))),
    //     ),
    // );
    let graph = add(
        x1.clone(),
        mul(
            x2.clone(),
            sin(add(x2.clone(), pow_f32(x3.clone(), Node::Variable(3f32)))),
        ),
    );
    x1.set(1f32);
    x2.set(2f32);
    x3.set(3f32);
    let mut result = graph.compute();
    result = round(result, 5);
    println!("Graph output = {}", result);
    assert_eq!(round(result, 5), -0.32727);
    x1.set(2f32);
    x2.set(3f32);
    x3.set(4f32);
    result = graph.compute();
    result = round(result, 5);
    println!("Graph output = {}", result);
    assert_eq!(round(result, 5), -0.56656);
}

enum Node {
    Computable(ComputationNode),
    Variable(f32),
}

impl Node {
    fn compute(&self) -> f32 {
        match self {
            Node::Computable(n) => n.compute(),
            Node::Variable(v) => *v,
        }
    }

    fn set(&self, x: f32) {
        self = &Node::Variable(x);
    }
}

struct ComputationNode {
    input: Vec<Node>,
    computation: Box<dyn Fn(Vec<f32>) -> f32>,
}

impl ComputationNode {
    fn compute(&self) -> f32 {
        let v = self.input.iter().map(|x| x.compute()).collect();
        (self.computation)(v)
    }
}

fn create_input(x: f32) -> Node {
    Node::Variable(x)
}

fn add(first: Node, second: Node) -> Node {
    let v = vec![first, second];
    Node::Computable(ComputationNode {
        input: v,
        computation: Box::new(|a: Vec<f32>| a[0] + a[1]),
    })
}

fn mul(first: Node, second: Node) -> Node {
    let v = vec![first, second];
    Node::Computable(ComputationNode {
        input: v,
        computation: Box::new(|a: Vec<f32>| a[0] * a[1]),
    })
}

fn sin(x: Node) -> Node {
    let v = vec![x];
    Node::Computable(ComputationNode {
        input: v,
        computation: Box::new(|a: Vec<f32>| a[0].sin()),
    })
}

fn pow_f32(first: Node, second: Node) -> Node {
    let v = vec![first, second];
    Node::Computable(ComputationNode {
        input: v,
        computation: Box::new(|a: Vec<f32>| a[0].powf(a[1])),
    })
}
