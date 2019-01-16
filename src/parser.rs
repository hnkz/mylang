use super::ast::{
    Ast, Statement, Arithmetic, RightNode, Node, NodeType, Number,
    Operator,
};
use super::tokenizer::{ Token, TokenType };

#[derive(Debug)]
pub struct Parser {
    index: usize,
    tokens: Vec<Token>,
    len: usize,
    state: ParseState,
    left_paren_count: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum ParseState {
    Normal,
    Arithmetic,
}

/*
    Statement   := Arithmetic ... First { (, Number, - }
    Arithmetic  := Node RightNode | Node
    RightNode   := Op Arithmetic
    Node        := (Arithmetic) | Number | -Number
*/
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let len = tokens.len();
        Parser {
            index: 0usize,
            tokens: tokens,
            len: len,
            state: ParseState::Normal,
            left_paren_count: 0u32,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Box<dyn Ast>>, String> {
        let mut statements: Vec<Statement> = Vec::new();
        let mut statement: Statement;
        let mut arithmetic: Arithmetic;
    
        while let Some(token) = self.now() {
            // end of the file
            if token.get_t_type() == TokenType::EOF {
                break;
            }

            statement = match self.get_statement() {
                Ok(statement) => statement,
                Err(err) => return Err(err)
            };
            statements.push(statement);
        }

        Err(format!("test"))
    }

    fn get_statement(&mut self) -> Result<Statement, String> {
        while let Some(token) = self.now() {
            match token.get_t_type() {
                TokenType::Number => {

                }
                TokenType::LeftParenthesis => {

                }
                TokenType::Minus => {

                }
                _ => {
                    return Err(format!("{:?} is not first token of Statement.", token.get_t_type()));
                }
            }
        }
        Err(format!("test"))
    }

    fn get_arithmetic(&mut self) -> Result<Arithmetic, String> {
        while let Some(token) = self.next() {
            match self.state {
                ParseState::Normal => {

                }
                ParseState::Arithmetic => {

                }
            }
        }

        Err(format!(""))
    }

    fn get_node(&mut self) -> Result<Node, String> {
        // To get number...Childish implementation...
        self.before();
        while let Some(token) = self.next() {
            match self.state {
                ParseState::Normal => {
                    let number = Number::new(token.get_inner());
                    let node = Node::new(NodeType::Number, Box::new(number));
                    
                }
                ParseState::Arithmetic => {

                }
            }
        }

        Err(format!(""))
    }

    fn now(&self) -> Option<Token> {
        if self.index < self.len {
            Some(self.tokens[self.index].clone())
        } else {
            None
        }
    }

    fn next(&mut self) -> Option<Token> {
        let res: Option<Token>;
        if self.index + 1 < self.len {
            self.index += 1;
            res = Some(self.tokens[self.index].clone());
        } else {
            res = None;
        }

        res
    }

    fn before(&mut self) -> Option<Token> {
        if self.index > 0 {
            self.index -= 1;
            Some(self.tokens[self.index].clone())
        } else {
            None
        }
    }
}