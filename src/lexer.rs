#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(i64),
    Text(String), // 修正：String -> Text
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

    pub fn next_token(&mut self) -> Option<Result<Token, String>> {
        while let Some(ch) = self.next_char() {
            match ch {
                ' ' | '\t' | '\r' | '\n' => continue,
                '=' => return Some(Ok(Token::Equals)),
                '+' => return Some(Ok(Token::Plus)),
                '-' => return Some(Ok(Token::Minus)),
                '*' => return Some(Ok(Token::Multiply)),
                '(' => return Some(Ok(Token::LeftParen)),
                ')' => return Some(Ok(Token::RightParen)),
                '{' => return Some(Ok(Token::LeftBrace)),
                '}' => return Some(Ok(Token::RightBrace)),
                ',' => return Some(Ok(Token::Comma)),
                ':' => return Some(Ok(Token::Colon)),
                '"' => {
                    let mut text = String::new();
                    while let Some(next) = self.next_char() {
                        if next == '"' {
                            break;
                        }
                        text.push(next);
                    }
                    if self.peek_char().is_none() {
                        return Some(Err("Unterminated string literal".to_string()));
                    }
                    return Some(Ok(Token::Text(text)));
                }
                c if c.is_ascii_digit() => {
                    let mut number = c.to_string();
                    while let Some(next) = self.peek_char() {
                        if next.is_ascii_digit() {
                            number.push(next);
                            self.next_char();
                        } else {
                            break;
                        }
                    }
                    return Some(Ok(Token::Number(number.parse().unwrap())));
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
                    return Some(match identifier.as_str() {
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
                    });
                }
                _ => return Some(Err(format!("Unexpected character: {}", ch))),
            }
        }
        None
    }
}
