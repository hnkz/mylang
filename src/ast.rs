
pub trait Ast {
    fn check_semantic(&mut self);
    fn generate_code(&mut self);
}

#[derive(Debug)]
pub struct Statement {

}

#[derive(Debug)]
pub struct Arithmetic {

}

#[derive(Debug)]
pub struct RightNode {

}

pub struct Operator {

}

#[derive(Debug)]
pub struct Node {

}

#[derive(Debug)]
pub enum NodeType {
    Int,
    Arithmetic,
}

impl Statement {
    pub fn new() -> Statement {
        Statement {

        }
    }
}

impl Arithmetic {
    pub fn new() -> Arithmetic {
        Arithmetic {
            
        }
    }
}

impl RightNode {
    pub fn new() -> RightNode {
        RightNode {
            
        }
    }
}

impl Operator {
    pub fn new() -> Operator {
        Operator {
            
        }
    }
}

impl Node {
    pub fn new() -> Node {
        Node {
            
        }
    }
}