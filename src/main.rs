// round to decimal digits
use std::cell::RefCell;

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
    let x4 = create_input(3f32);
    // graph variable is the output node of the graph:
    // let graph = add(
    //     x1.clone(),
    //     mul(
    //         x2.clone(),
    //         sin(add(x2.clone(), pow_f32(x3.clone(), Node::Variable(3f32)))),
    //     ),
    // );

    let pow = pow_f32(&x3, &x4);
    let add1 = add(&x2, &pow);
    let sin = sin(&add1);
    let mul = mul(&x2, &sin);
    let graph = add(&x1, &mul);

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

struct Node<'a> {
    node_type: RefCell<NodeType<'a>>,
}

enum NodeType<'a> {
    Computable(ComputationNode<'a>),
    Variable(f32),
}

impl Node<'_> {
    fn compute(&self) -> f32 {
        match &*self.node_type.borrow() {
            NodeType::Computable(n) => n.compute(),
            NodeType::Variable(v) => *v,
        }
    }

    fn set(&self, x: f32) {
        self.node_type.replace(NodeType::Variable(x));
    }
}

struct ComputationNode<'a> {
    input: Vec<&'a Node<'a>>,
    computation: Box<dyn Fn(Vec<f32>) -> f32>,
}

impl ComputationNode<'_> {
    fn compute(&self) -> f32 {
        let v = self.input.iter().map(|x| x.compute()).collect();
        (self.computation)(v)
    }
}

fn create_input<'a>(x: f32) -> Node<'a> {
    Node {
        node_type: RefCell::new(NodeType::Variable(x)),
    }
}

fn add<'a>(first: &'a Node<'a>, second: &'a Node<'a>) -> Node<'a> {
    let v = vec![first, second];
    Node {
        node_type: RefCell::new(NodeType::Computable(ComputationNode {
            input: v,
            computation: Box::new(|a: Vec<f32>| a[0] + a[1]),
        })),
    }
}

fn mul<'a>(first: &'a Node<'a>, second: &'a Node<'a>) -> Node<'a> {
    let v = vec![first, second];
    Node {
        node_type: RefCell::new(NodeType::Computable(ComputationNode {
            input: v,
            computation: Box::new(|a: Vec<f32>| a[0] * a[1]),
        })),
    }
}

fn sin<'a>(x: &'a Node<'a>) -> Node<'a> {
    let v = vec![x];
    Node {
        node_type: RefCell::new(NodeType::Computable(ComputationNode {
            input: v,
            computation: Box::new(|a: Vec<f32>| a[0].sin()),
        })),
    }
}

fn pow_f32<'a>(first: &'a Node<'a>, second: &'a Node<'a>) -> Node<'a> {
    let v = vec![first, second];
    Node {
        node_type: RefCell::new(NodeType::Computable(ComputationNode {
            input: v,
            computation: Box::new(|a: Vec<f32>| a[0].powf(a[1])),
        })),
    }
}
