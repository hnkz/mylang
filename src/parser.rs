use super::ast::{ Ast, Statement, Arithmetic, RightNode, Node, NodeType };
use super::tokenizer::{ Token, TokenType };

#[derive(Debug)]
pub struct Parser {
    index: usize,
    tokens: Vec<Token>,
    len: usize,
}

#[derive(Debug)]
pub enum ParseState {
    Normal,
    Statement,
}

/*
    Statement   := Arithmetic
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
        }
    }

    pub fn parse(&mut self) -> Result<Box<dyn Ast>, String> {
        let mut root = Statement::new();
        let mut arithmetic = Arithmetic::new();

        let mut state = ParseState::Normal;
        while let Some(token) = self.next() {
            match state {
                ParseState::Normal => {
                    match token.get_t_type() {
                        TokenType::Number => {
                            
                        }
                        TokenType::LeftParenthesis => {

                        }
                        _ => {
                            return Err(format!("Parser is stop!!"));
                        }
                    }
                }
                ParseState::Statement => {

                }
            }
        }

        Err(format!("test"))
    }

    fn now(&self) -> &Token {
        &self.tokens[self.index]
    }

    fn next(&mut self) -> Option<&Token> {
        let i = self.index;
        if i < self.len {
            self.index += 1;
            Some(&self.tokens[i])
        } else {
            None
        } 
    }

    fn before(&mut self) -> Option<&Token> {
        if self.index > 0 {
            self.index -= 1;
            Some(&self.tokens[self.index])
        } else {
            None
        }
    }
}