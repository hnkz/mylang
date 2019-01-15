
#[derive(Debug)]
pub struct Tokenizer {
    code: Vec<char>,
    len: usize,
}

#[derive(Debug)]
pub enum TokenizeState {
    Normal,
    Number,
    Plus,
    Minus,
    Asterisk,
    Slash,
    NewLine,
    LeftParenthesis,
    RightParenthesis,
}

#[derive(Debug)]
pub struct Token {
    t_type: TokenType,
    content: String,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Number,
    Dot,
    Equal,
    Str,
    NewLine,
    LeftParenthesis,
    RightParenthesis,
}

impl Tokenizer {
    pub fn new(code: Vec<char>) -> Tokenizer {
        let len = code.len();
        Tokenizer {
            code: code,
            len: len 
        }
    }

    pub fn tokenize(&self) -> Result<Vec<Token>, String> {
        let mut state  = TokenizeState::Normal;
        let mut tmp_contents = String::new();
        let mut tokens: Vec<Token> = Vec::new();

        let code = &self.code;
        let mut  i = 0;
        while i < self.len {
            let c = code[i];
            match state {
                TokenizeState::Normal => {
                    match c {
                        '0'...'9' => {
                            state = TokenizeState::Number;
                            tmp_contents.push(c);
                        }
                        '+' => {
                            state = TokenizeState::Plus;
                            tmp_contents.push(c);
                        }
                        '-' => {
                            state = TokenizeState::Minus;
                            tmp_contents.push(c);
                        }
                        '*' => {
                            state = TokenizeState::Asterisk;
                            tmp_contents.push(c);
                        }
                        '/' => {
                            state = TokenizeState::Slash;
                            tmp_contents.push(c);
                        }
                        '\n' => {
                            state = TokenizeState::NewLine;
                        }
                        '(' => {
                            state = TokenizeState::LeftParenthesis;
                            tmp_contents.push(c);
                        }
                        ')' => {
                            state = TokenizeState::RightParenthesis;
                            tmp_contents.push(c);
                        }
                        _ => {}
                    }
                }
                TokenizeState::Number => {
                    match c {
                        '0'...'9' => {
                            tmp_contents.push(c);
                        }
                        _ => {
                            i -= 1;
                            let token = Token::new(TokenType::Number,  tmp_contents.clone());
                            tmp_contents.clear();
                            tokens.push(token);
                            state = TokenizeState::Normal;
                        }
                    }
                }
                TokenizeState::Plus => {
                    i -= 1;
                    let token = Token::new(TokenType::Plus,  tmp_contents.clone());
                    tmp_contents.clear();
                    tokens.push(token);
                    state = TokenizeState::Normal;
                }
                TokenizeState::Minus => {
                    i -= 1;
                    let token = Token::new(TokenType::Minus,  tmp_contents.clone());
                    tmp_contents.clear();
                    tokens.push(token);
                    state = TokenizeState::Normal;
                }
                TokenizeState::Asterisk => {
                    i -= 1;
                    let token = Token::new(TokenType::Asterisk,  tmp_contents.clone());
                    tmp_contents.clear();
                    tokens.push(token);
                    state = TokenizeState::Normal;
                }
                TokenizeState::Slash => {
                    i -= 1;
                    let token = Token::new(TokenType::Slash,  tmp_contents.clone());
                    tmp_contents.clear();
                    tokens.push(token);
                    state = TokenizeState::Normal;
                }
                TokenizeState::NewLine => {
                    i -= 1;
                    let token = Token::new(TokenType::NewLine,  tmp_contents.clone());
                    tmp_contents.clear();
                    tokens.push(token);
                    state = TokenizeState::Normal;
                }
                TokenizeState::LeftParenthesis => {
                    i -= 1;
                    let token = Token::new(TokenType::LeftParenthesis,  tmp_contents.clone());
                    tmp_contents.clear();
                    tokens.push(token);
                    state = TokenizeState::Normal;
                }
                TokenizeState::RightParenthesis => {
                    i -= 1;
                    let token = Token::new(TokenType::RightParenthesis,  tmp_contents.clone());
                    tmp_contents.clear();
                    tokens.push(token);
                    state = TokenizeState::Normal;
                }
            }

            i += 1;
        }

        match state {
            TokenizeState::Normal | TokenizeState::NewLine => {}
            TokenizeState::Number => {
                let token = Token::new(TokenType::Number,  tmp_contents.clone());
                tokens.push(token);
            }
            _ => {
                return Err(format!("Tokenize state is incorrect."));
            }
        }

        Ok(tokens)
    }
}

impl Token {
    fn new(t_type: TokenType, content: String) -> Token {
        Token {
            t_type: t_type,
            content: content,
        }
    }

    pub fn get_t_type(&self) -> TokenType {
        self.t_type
    }
}