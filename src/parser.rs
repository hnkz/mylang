use super::ast::{
    Ast, Statement, Arithmetic, Node, Number,
    Operator,
};
use super::tokenizer::{ Token, TokenType };

#[derive(Debug)]
pub struct Parser {
    index: usize,
    tokens: Vec<Token>,
    len: usize,
    state: ParseState,
    open_paren_count: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum ParseState {
    Normal,
    Arithmetic,
}

/*
    Statement   := Arithmetic
    Arithmetic  := Node Op Node | Node
    Node        := (Arithmetic) | Arithmetic | Number | -Number
*/
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let len = tokens.len();
        Parser {
            index: 0usize,
            tokens: tokens,
            len: len,
            state: ParseState::Normal,
            open_paren_count: 0u32,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements: Vec<Statement> = Vec::new();
    
        loop {
            let token = self.now();
            // end of the file
            if token.get_t_type() == TokenType::EOF || token.get_t_type() == TokenType::NewLine {
                break;
            }

            let statement = match self.get_statement() {
                Ok(statement) => statement,
                Err(err) => return Err(err)
            };
            statements.push(statement);
        }

        Ok(statements)
    }

    fn get_statement(&mut self) -> Result<Statement, String> {
        let statement: Statement;
        let token = self.now();

        let arithmetic = match self.get_arithmetic() {
            Ok(arithmetic) => arithmetic,
            Err(err) => return Err(err),
        };
        statement = Statement::Arithmetic(arithmetic);

        if self.open_paren_count > 0 {
            return Err(format!("({}, {}) Parenthesis does not match.", token.get_line(), token.get_x()));
        }
        
        Ok(statement)
    }

    fn get_arithmetic(&mut self) -> Result<Arithmetic, String> {
        let mut arithmetic: Arithmetic;

        let l_node = match self.get_node() {
            Ok(node) => node,
            Err(err) => return Err(err),
        };

        arithmetic = Arithmetic::Term(l_node);

        if !self.is_end_of_statement() {
            loop {
                let token = self.now();
                let op = match token.get_t_type() {
                    TokenType::Plus => Operator::Plus,
                    TokenType::Minus => Operator::Minus,
                    TokenType::Asterisk => Operator::Mul,
                    TokenType::Slash => Operator::Div,
                    _ => break,
                };

                self.next();

                let r_node = match self.get_node() {
                    Ok(node) => node,
                    Err(err) => return Err(err),
                };

                arithmetic = match op {
                    Operator::Plus | Operator::Minus => {
                        arithmetic.insert_top(op, r_node)
                    }
                    Operator::Mul | Operator::Div => {
                        arithmetic.insert_right(op, r_node)
                    }
                };

            }
        };
        
        Ok(arithmetic)
    }

    fn get_node(&mut self) -> Result<Node, String> {
        let node: Node;
        let token = self.now();

        match token.get_t_type() {
            TokenType::Number  => {
                let number = Number::new(token.get_inner());
                node = Node::Number(number);

                self.next();
            }
            TokenType::LeftParenthesis => {
                self.next();
                self.inc_open_paren_count();
                let arithmetic = match self.get_arithmetic() {
                    Ok(arithmetic) => arithmetic,
                    Err(err) => return Err(err),
                };
                node = Node::Arithmetic(Box::new(arithmetic));

                let token = self.now();
                
                if token.get_t_type() == TokenType::RightParenthesis {
                    match self.dec_open_paren_count() {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(err);
                        }
                    };
                    self.next();
                }
            }
            // On the way
            TokenType::Minus => {
                // Skip Minus
                self.next();
                let token = if let Some(token) = self.next() {
                    token
                } else {
                    return Err(format!("File is end in the way of Parsing."));
                };

                return Err(format!(""));
            }
            _ => {
                return Err(format!("({}, {}) {:?} is not first token of Node.", token.get_line(), token.get_x(), token.get_inner()));
            }
        }

        Ok(node)
    }

    fn is_end_of_statement(&self) -> bool {
        let token = self.now();
        match token.get_t_type() {
            TokenType::EOF | TokenType::NewLine => {
                true
            }
            _ => {
                false
            }
        }
    }

    fn now(&self) -> Token {
        self.tokens[self.index].clone()
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

    // Increment Open parenthesis count
    fn inc_open_paren_count(&mut self) {
        self.open_paren_count += 1;
    }

    // Decrement Open parenthesis count
    fn dec_open_paren_count(&mut self) -> Result<(), String> {
        if self.open_paren_count == 0 {
            let token = self.now();
            return Err(format!("({}, {}) Parenthethis does not match.", token.get_line(), token.get_x()));
        }
        self.open_paren_count -= 1;

        Ok(())
    }
}