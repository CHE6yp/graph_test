use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

pub struct Node {
    node_type: RefCell<NodeType>,
}

impl Node {
    fn create_computable(input: Vec<Rc<Node>>, computation: Box<dyn Fn(Vec<f32>) -> f32>) -> Node {
        Node {
            node_type: RefCell::new(NodeType::Computable(ComputationNode {
                input,
                computation,
                cache: RefCell::new(BTreeMap::new()),
            })),
        }
    }

    pub fn compute(&self) -> f32 {
        match &*self.node_type.borrow() {
            NodeType::Computable(n) => n.compute(),
            NodeType::Variable(v) => *v,
        }
    }

    pub fn set(&self, x: f32) {
        match *self.node_type.borrow_mut() {
            NodeType::Computable(_) => panic!("Cannot set a computable node"),
            NodeType::Variable(ref mut v) => *v = x,
        };
    }
}

enum NodeType {
    Computable(ComputationNode),
    Variable(f32),
}

struct ComputationNode {
    input: Vec<Rc<Node>>,
    computation: Box<dyn Fn(Vec<f32>) -> f32>,
    cache: RefCell<BTreeMap<String, f32>>,
}

impl ComputationNode {
    fn compute(&self) -> f32 {
        let v = self.input.iter().map(|x| x.compute()).collect();
        let key = format!("{:?}", v);
        if self.cache.borrow().contains_key(&key) {
            println!("Getting cached value");
            return *self.cache.borrow().get(&key).unwrap();
        }
        println!("Computing");
        let result = (self.computation)(v);
        self.cache.borrow_mut().insert(key, result);
        result
    }
}

pub fn create_input<'a>(x: f32) -> Rc<Node> {
    Rc::new(Node {
        node_type: RefCell::new(NodeType::Variable(x)),
    })
}

pub fn add(augend: Rc<Node>, addend: Rc<Node>) -> Rc<Node> {
    Rc::new(Node::create_computable(
        vec![augend, addend],
        Box::new(|a: Vec<f32>| a[0] + a[1]),
    ))
}

pub fn sub(minuend: Rc<Node>, subtrahend: Rc<Node>) -> Rc<Node> {
    Rc::new(Node::create_computable(
        vec![minuend, subtrahend],
        Box::new(|a: Vec<f32>| a[0] - a[1]),
    ))
}

pub fn mul(multiplier: Rc<Node>, multiplicand: Rc<Node>) -> Rc<Node> {
    Rc::new(Node::create_computable(
        vec![multiplier, multiplicand],
        Box::new(|a: Vec<f32>| a[0] * a[1]),
    ))
}

pub fn div(dividend: Rc<Node>, divisor: Rc<Node>) -> Rc<Node> {
    Rc::new(Node::create_computable(
        vec![dividend, divisor],
        Box::new(|a: Vec<f32>| a[0] / a[1]),
    ))
}

pub fn sin(angle: Rc<Node>) -> Rc<Node> {
    Rc::new(Node::create_computable(
        vec![angle],
        Box::new(|a: Vec<f32>| a[0].sin()),
    ))
}

pub fn pow_f32(base: Rc<Node>, exponent: Rc<Node>) -> Rc<Node> {
    Rc::new(Node::create_computable(
        vec![base, exponent],
        Box::new(|a: Vec<f32>| a[0].powf(a[1])),
    ))
}
