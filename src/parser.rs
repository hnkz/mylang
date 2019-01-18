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
    Statement   := Arithmetic ... First { (, Number, - }
    Arithmetic  := Node RightNode | Node ... First { (, Number, -)}
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
            open_paren_count: 0u32,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements: Vec<Statement> = Vec::new();
    
        while let Some(token) = self.now() {
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
        println!("statement");
        let statement: Statement;
        let token = if let Some(token) = self.now() {
            token
        } else {
            return Err(format!("End of file is not EOF Token."));
        };

        let arithmetic = match self.get_arithmetic() {
            Ok(arithmetic) => arithmetic,
            Err(err) => return Err(err),
        };
        statement = Statement::Arithmetic(arithmetic);
        
        Ok(statement)
    }

    fn get_arithmetic(&mut self) -> Result<Arithmetic, String> {
        println!("arithmetic");
        let arithmetic: Arithmetic;

        let l_node = match self.get_node() {
            Ok(node) => node,
            Err(err) => return Err(err),
        };

        arithmetic = if self.is_end_of_statement() {
            Arithmetic::Term(l_node)
        } else {
            let mut arithmetic: Arithmetic;

            let token = if let Some(token) =  self.now() {
                token
            } else {
                return Err(format!("Token is dead."));
            };

            let op = match token.get_t_type() {
                TokenType::Plus => Operator::Plus,
                TokenType::Minus => Operator::Minus,
                TokenType::Asterisk => Operator::Mul,
                TokenType::Slash => Operator::Div,
                _ => return Err(format!("{:?} is not Operator.", token.get_inner())),
            };

            self.next();

            let r_node = match self.get_node() {
                Ok(node) => node,
                Err(err) => return Err(err),
            }; 

            arithmetic = Arithmetic::MultiTerm(l_node, op, r_node);

            while let Some(token) = self.now() {
                if self.is_end_of_statement() {
                    break;
                }

                let op = match token.get_t_type() {
                    TokenType::Plus => Operator::Plus,
                    TokenType::Minus => Operator::Minus,
                    TokenType::Asterisk => Operator::Mul,
                    TokenType::Slash => Operator::Div,
                    _ => return Err(format!("{:?} is not Operator.", token.get_inner())),
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

            arithmetic
        };
        
        Ok(arithmetic)
    }

    fn get_node(&mut self) -> Result<Node, String> {
        println!("node");
        let node: Node;
        let token = if let Some(token) = self.now() {
            token
        } else {
            return Err(format!("End of file is not EOF Token."));
        };

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

                let token = if let Some(token) = self.now() {
                    token
                } else {
                    return Err(format!("End of file is not EOF Token."));
                };

                // skip right parenthesis
                if token.get_t_type() != TokenType::RightParenthesis {
                    return Err(format!("Parenthethis does not match."));
                } else {
                    self.dec_open_paren_count();
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
                    return Err(format!("End of file is not EOF Token."));
                };

                return Err(format!(""));
            }
            _ => {
                return Err(format!("{:?} is not first token of Node.", token.get_inner()));
            }
        }

        Ok(node)
    }

    fn is_end_of_statement(&self) -> bool {
        if let Some(token) = self.now() {
            match token.get_t_type() {
                TokenType::EOF | TokenType::NewLine => {
                    true
                }
                _ => {
                    false
                }
            }
        } else {
            true
        }
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

    // Increment Open parenthesis count
    fn inc_open_paren_count(&mut self) {
        self.open_paren_count += 1;
    }

    // Decrement Open parenthesis count
    fn dec_open_paren_count(&mut self) -> Result<(), String> {
        if self.open_paren_count == 0 {
            return Err(format!("Parenthethis does not match."));
        }
        self.open_paren_count -= 1;

        Ok(())
    }
}