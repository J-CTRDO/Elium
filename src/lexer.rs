#[derive(Debug, PartialEq)]
pub enum Token {
    Print,
    Set,
    Identifier(String),
    Number(i64),
    Plus,
    Minus,
    Equals,
    Unknown(char),
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
// コンストラクタ
    pub fn new(input: &str) -> Self {
        Self {
        input: input.chars().collect(),
        position: 0,
        }
    }

// 次の文字を取得
    fn next_char(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let ch = self.input[self.position];
            self.position += 1;
            Some(ch)
        } else {
            None
        }
    }

// トークンの解析
    pub fn next_token(&mut self) -> Option<Token> {
        while let Some(ch) = self.next_char() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => continue, // 空白は無視
                '=' => return Some(Token::Equals),
                '+' => return Some(Token::Plus),
                '-' => return Some(Token::Minus),
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
                    return Some(Token::Number(number.parse().unwrap()));
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
                        "PRINT" => Some(Token::Print),
                        "SET" => Some(Token::Set),
                        _ => Some(Token::Identifier(identifier)),
                    };
                }
                unknown => return Some(Token::Unknown(unknown)),
            }
        }
        None
    }    // 次の文字を覗き見る                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   
    fn peek_char(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
        
    }
            
}

