#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(i64),
    String(String),

    Plus,
    Minus,
    Multiply,
    Equals,
    GreaterThan,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Colon,

    Package,
    Import,
    From,
    To,
    Msg,
    If,
    Else,
    Function,
    Return,
    Exit,
    Input,
    Get,
    Async,

    Unknown(char),
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let ch = self.input[self.position];
            self.position += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(ch)
        } else {
            None
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        while let Some(ch) = self.next_char() {
            match ch {
                ' ' | '\t' | '\r' | '\n' => continue,
                '=' => return Ok(Token::Equals),
                '+' => return Ok(Token::Plus),
                '-' => return Ok(Token::Minus),
                '*' => return Ok(Token::Multiply),
                '(' => return Ok(Token::LeftParen),
                ')' => return Ok(Token::RightParen),
                '{' => return Ok(Token::LeftBrace),
                '}' => return Ok(Token::RightBrace),
                ',' => return Ok(Token::Comma),
                ':' => return Ok(Token::Colon),
                '"' => {
                    let mut string = String::new();
                    while let Some(next) = self.peek_char() {
                        if next == '"' {
                            self.next_char(); // Consume closing quote
                            break;
                        } else {
                            string.push(next);
                            self.next_char();
                        }
                    }
                    return Ok(Token::String(string));
                }
                c if c.is_digit(10) => {
                    let mut number = c.to_string();
                    while let Some(next) = self.peek_char() {
                        if next.is_digit(10) {
                            number.push(next);
                            self.next_char();
                        } else {
                            break;
                        }
                    }
                    return Ok(Token::Number(number.parse().unwrap()));
                }
                c if c.is_alphabetic() => {
                    let mut identifier = c.to_string();
                    while let Some(next) = self.peek_char() {
                        if next.is_alphanumeric() {
                            identifier.push(next);
                            self.next_char();
                        } else {
                            break;
                        }
                    }
                    return match identifier.as_str() {
                        "package" => Ok(Token::Package),
                        "import" => Ok(Token::Import),
                        "from" => Ok(Token::From),
                        "to" => Ok(Token::To),
                        "msg" => Ok(Token::Msg),
                        "if" => Ok(Token::If),
                        "else" => Ok(Token::Else),
                        "function" => Ok(Token::Function),
                        "return" => Ok(Token::Return),
                        "exit" => Ok(Token::Exit),
                        "input" => Ok(Token::Input),
                        "get" => Ok(Token::Get),
                        "async" => Ok(Token::Async),
                        _ => Ok(Token::Identifier(identifier)),
                    };
                }
                _ => return Ok(Token::Unknown(ch)),
            }
        }
        Err("End of input".to_string())
    }
}
