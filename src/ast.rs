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
pub struct Arithmetic {
    left: Node,
    right: RightNode,
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
pub enum Node {
    Number(Number),
    Arithmetic(Box<Arithmetic>),
}

#[derive(Debug)]
pub struct Number {
    inner: String,
}

impl Arithmetic {
    pub fn new(left: Node, right: RightNode) -> Arithmetic {
        Arithmetic {
            left: left, 
            right: right,
        }
    }
}

impl RightNode {
    pub fn new(op: Operator, node: Node) -> RightNode {
        RightNode {
            op: op,
            node: node,
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

impl Ast for Statement {
    fn check_semantic(&mut self) {

    }

    fn generate_code(&mut self) {
        println!("  .text");
        println!(".global _main");
        println!("");
        println!("_main:");
        match self {
            Statement::Arithmetic(arithmetic) => {
                arithmetic.generate_code();
            }
        }
        println!("  ret");
    }
}

impl Ast for Arithmetic {
    fn check_semantic(&mut self) {

    }

    fn generate_code(&mut self) {
        self.left.generate_code();
        self.right.generate_code();
    }
}

impl Ast for Node {
    fn check_semantic(&mut self) {

    }

    fn generate_code(&mut self) {
        match self {
            Node::Arithmetic(arithmetic) => {
                arithmetic.generate_code();
            }
            Node::Number(number) => {
                number.generate_code();
            }
        }
    }
}

impl Ast for RightNode {
    fn check_semantic(&mut self) {

    }

    fn generate_code(&mut self) {
        self.node.generate_code();
        self.op.generate_code();
    }
}

impl Ast for Operator {
    fn check_semantic(&mut self) {

    }

    fn generate_code(&mut self) {
        println!("  pop  %rax");
        println!("  pop  %rbx");
        match self {
            Operator::Plus => {
                println!("  add %rbx, %rax");
            }
            Operator::Minus => {
                println!("  sub %rbx, %rax");
            }
            Operator::Mul => {
                println!("  mul %rbx");
            }
            Operator::Div => {
                println!("  div %rbx");
            }
        }
    }
}

impl Ast for Number {
    fn check_semantic(&mut self) {

    }
    
    fn generate_code(&mut self) {
        println!("  push ${}", self.inner);
    }
}