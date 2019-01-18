use std::fmt::Debug;

pub trait Ast: Debug {
    fn check_semantic(&mut self);
    fn generate_code(&mut self);
}

#[derive(Debug)]
pub struct Program {
    statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Arithmetic(Arithmetic),
}

#[derive(Debug, Clone)]
pub enum Arithmetic {
    Term(Node),
    MultiTerm(Node, Operator, Node),
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum Node {
    Number(Number),
    Arithmetic(Box<Arithmetic>),
}

#[derive(Debug, Clone)]
pub struct Number {
    inner: String,
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }

    pub fn push(&mut self, statement: Statement) {
        self.statements.push(statement)
    }
}

impl Arithmetic {
    pub fn insert_right(&mut self, newop: Operator, r_node: Node) -> Arithmetic {
        match self {
            Arithmetic::Term(term) => {
                Arithmetic::MultiTerm(term.clone(), newop, r_node)
            }
            Arithmetic::MultiTerm(left, op, right) => {
                Arithmetic::MultiTerm(left.clone(), op.clone(), Node::Arithmetic(Box::new(Arithmetic::MultiTerm(right.clone(), newop, r_node))))
            }
        }
    }

    pub fn insert_top(&mut self, newop: Operator, r_node: Node) -> Arithmetic {
        match self {
            Arithmetic::Term(term) => {
                Arithmetic::MultiTerm(term.clone(), newop, r_node)
            }
            Arithmetic::MultiTerm(left, op, right) => {
                Arithmetic::MultiTerm(Node::Arithmetic(Box::new(self.clone())), newop, r_node)
            }
        }
    }

    pub fn get_op_type(&self) -> Option<Operator> {
        match self {
            Arithmetic::Term(_) => {
                None
            }
            Arithmetic::MultiTerm(_, op, _) => {
                Some(*op)
            }
        }
    }
}

impl Operator {
    
}

impl Number {
    pub fn new(inner: String) -> Number {
        Number {
            inner: inner
        }
    }
}

impl Ast for Program {
    fn check_semantic(&mut self) {

    }

    fn generate_code(&mut self) {
        println!("  .text");
        println!(".global _main");
        println!("");
        println!("_main:");

        for statement in self.statements.iter_mut() {
            statement.generate_code();
        }

        println!("  ret");
    }
}

impl Ast for Statement {
    fn check_semantic(&mut self) {

    }

    fn generate_code(&mut self) {
        match self {
            Statement::Arithmetic(arithmetic) => {
                arithmetic.generate_code();
            }
        }
        println!("  pop %rax");
    }
}

impl Ast for Arithmetic {
    fn check_semantic(&mut self) {

    }

    fn generate_code(&mut self) {
        match self {
            Arithmetic::Term(term) => {
                term.generate_code();
            }
            Arithmetic::MultiTerm(left, op, right) => {
                left.generate_code();
                right.generate_code();
                op.generate_code();
            }
        }
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

impl Ast for Operator {
    fn check_semantic(&mut self) {

    }

    fn generate_code(&mut self) {
        println!("  pop  %rbx");
        println!("  pop  %rax");
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

        println!("  push %rax");
    }
}

impl Ast for Number {
    fn check_semantic(&mut self) {

    }
    
    fn generate_code(&mut self) {
        println!("  push ${}", self.inner);
    }
}