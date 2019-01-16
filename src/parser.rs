use super::ast::{
    Ast, Statement, Arithmetic, RightNode, Node, Number,
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

        match token.get_t_type() {
            TokenType::Number | TokenType::LeftParenthesis | TokenType::Minus => {
                let arithmetic = match self.get_arithmetic() {
                    Ok(arithmetic) => arithmetic,
                    Err(err) => return Err(err),
                };
                statement = Statement::Arithmetic(arithmetic);
            }
            _ => {
                return Err(format!("{:?} is not first token of Statement.", token.get_inner()));
            }
        }
        
        Ok(statement)
    }

    fn get_arithmetic(&mut self) -> Result<Arithmetic, String> {
        println!("arithmetic");
        let arithmetic: Arithmetic;
        let token = if let Some(token) = self.now() {
            token
        } else {
            return Err(format!("End of file is not EOF Token."));
        };
        match token.get_t_type() {
            TokenType::Number => {
                let node = match self.get_node() {
                    Ok(node) => node,
                    Err(err) => return Err(err),
                };
                
                let right_node = match self.get_rightnode() {
                    Ok(right_node) => right_node,
                    Err(err) => return Err(err),
                };

                arithmetic = Arithmetic::new(node, right_node);
            }
            TokenType::LeftParenthesis => {
                return Err(format!(""));
            }
            TokenType::Minus => {
                return Err(format!(""));
            }
            _ => {
                return Err(format!("{:?} is not first token of Statement.", token.get_inner()));
            }
        }
        
        Ok(arithmetic)
    }

    fn get_rightnode(&mut self) -> Result<RightNode, String> {
        println!("rightnode");
        let right_node: RightNode;
        let op: Operator;
        let op_token = if let Some(token) = self.now() {
            token
        } else {
            return Err(format!("End of file is not EOF Token."));
        };

        op = match op_token.get_t_type() {
            TokenType::Plus => Operator::Plus,
            TokenType::Minus => Operator::Minus,
            TokenType::Asterisk => Operator::Mul,
            TokenType::Slash => Operator::Div,
            _ => return Err(format!("{:?} is not Operator.", op_token.get_inner())),
        };

        self.next();
        let node = match self.get_node() {
            Ok(node) => node,
            Err(err) => return Err(err),
        };

        right_node = RightNode::new(op, node);
        
        Ok(right_node)
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
    fn dec_open_paren_count(&mut self) {
        self.open_paren_count -= 1;
    }
}