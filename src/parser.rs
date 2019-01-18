use super::ast::{
    Program, Statement, Arithmetic, Node, Number,
    Operator,
};
use super::tokenizer::{ Token, TokenType };

#[derive(Debug)]
pub struct Parser {
    index: usize,
    tokens: Vec<Token>,
    len: usize,
    state: ParseState,
    err_handler: Vec<String>,
    open_paren_count: u32,
}

#[derive(Debug)]
pub struct ParseErrorHandler {
    err: Vec<String>
}

#[derive(Debug, Clone, Copy)]
pub enum ParseState {
    Normal,
    Arithmetic,
}

/*
    Statement   := Arithmetic ;
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
            err_handler: Vec::new(),
            open_paren_count: 0u32,
        }
    }

    pub fn parse(&mut self) -> Result<Program, Vec<String>> {
        let mut program = Program::new();
    
        loop {
            // end of the file
            let token = self.now();
            if token.get_t_type() == TokenType::EOF {
                break;
            } else if token.get_t_type() == TokenType::NewLine {
                self.next();
                continue;
            }

            match self.get_statement() {
                Ok(statement) => program.push(statement),
                Err(err) => {
                    self.err_handler.push(err);
                    self.skip_to_next_statement();
                },
            };
        }

        if self.err_handler.len() == 0 {
            Ok(program)
        } else {
            Err(self.err_handler.clone())
        }
    }

    // Statement   := Arithmetic ;
    fn get_statement(&mut self) -> Result<Statement, String> {
        let statement: Statement;

        let arithmetic = match self.get_arithmetic() {
            Ok(arithmetic) => arithmetic,
            Err(err) => return Err(err),
        };

        let token = self.now();
        if token.get_t_type() != TokenType::Semicolon {
            return Err(ParseErrorHandler::create_error(token, "Statement should have Semicolon."));
        }
        self.next();

        statement = Statement::Arithmetic(arithmetic);
        
        Ok(statement)
    }

    // Arithmetic  := Node Op Node | Node
    fn get_arithmetic(&mut self) -> Result<Arithmetic, String> {
        let mut arithmetic: Arithmetic;

        let l_node = match self.get_node() {
            Ok(node) => node,
            Err(err) => return Err(err),
        };

        arithmetic = Arithmetic::Term(l_node);

        let t_type = self.now().get_t_type();
        if  t_type == TokenType::Plus || t_type == TokenType::Minus ||
            t_type == TokenType::Asterisk || t_type == TokenType::Slash
        {
            loop {
                let token = self.now();
                let op = match token.get_t_type() {
                    TokenType::Plus => Operator::Plus,
                    TokenType::Minus => Operator::Minus,
                    TokenType::Asterisk => Operator::Mul,
                    TokenType::Slash => Operator::Div,
                    _ => break,
                };

                // skip Operator
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
                } else {
                    return Err(ParseErrorHandler::create_error(token, "There isn't Close Parenthesis."));
                }
            }
            // On the way
            TokenType::Minus => {
                // Skip Minus
                self.next();
                let token = self.next();

                return Err(format!(""));
            }
            _ => {
                return Err(ParseErrorHandler::create_error(token, "is not first token of Node."));
            }
        }

        Ok(node)
    }

    fn skip_to_next_statement(&mut self) {
        let mut token = self.now();
        let mut t_type = token.get_t_type();
        while t_type != TokenType::NewLine && t_type != TokenType::EOF {
            token = self.next();
            t_type = token.get_t_type();
        }
        self.next();
    }

    fn now(&self) -> Token {
        self.tokens[self.index].clone()
    }

    fn next(&mut self) -> Token {
        if self.index + 1 < self.len {
            self.index += 1;
        }
        self.tokens[self.index].clone()
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
            return Err(ParseErrorHandler::create_error(token, "Parenthethis does not match."));
        }
        self.open_paren_count -= 1;

        Ok(())
    }
}

impl ParseErrorHandler {
    pub fn new() -> ParseErrorHandler {
        ParseErrorHandler {
            err: Vec::new(),
        }
    }

    pub fn create_error(token: Token, sentence: &str) -> String {
        format!("({}, {}) \"{:?} {}\" {}", token.get_line(), token.get_x(), token.get_t_type(), token.get_inner(), sentence)
    }
}