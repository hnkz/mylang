use super::tokenizer::Token;
use std::fmt::Debug;

pub trait Ast: Debug {
    fn check_semantic(&mut self);
    fn generate_code(&mut self);
}

#[derive(Debug)]
pub enum Statement {
    Arithmetic(Arithmetic),
}

#[derive(Debug)]
pub enum Arithmetic {
    Node(Node),
    OpRightNode((Node, RightNode)),
}

#[derive(Debug)]
pub struct RightNode {
    op: Operator,
    node: Node,
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
}

#[derive(Debug)]
pub struct Node {
    n_type: NodeType,
    inner: Box<dyn Ast>,
}

#[derive(Debug)]
pub struct Number {
    inner: String,
}

#[derive(Debug)]
pub enum NodeType {
    Number,
    Arithmetic,
}

impl RightNode {
    pub fn new(op: Operator, node: Node) -> RightNode {
        RightNode {
            op: op,
            node: node,
        }
    }
}

impl Node {
    pub fn new(n_type: NodeType, inner: Box<Ast>) -> Node {
        Node {
            n_type: n_type,
            inner: inner
        }
    }
}

impl Number {
    pub fn new(inner: String) -> Number {
        Number {
            inner: inner
        }
    }
}

impl Ast for Number {
    fn check_semantic(&mut self) {

    }
    
    fn generate_code(&mut self) {

    }
}