#[derive(Debug, PartialEq)]
pub enum Token {
    Print,
    Set,
    If,
    Else,
    Identifier(String),
    Number(i64),
    Plus,
    Minus,
    Multiply,
    Equals,
    Unknown(char),
}

#[derive(Debug)]
pub struct LexerError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    // コンストラクタ
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    // 次の文字を取得
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

    // 次の文字を覗き見る
    fn peek_char(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    // トークンの解析
    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        while let Some(ch) = self.next_char() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => continue, // 空白は無視
                '=' => return Ok(Token::Equals),
                '+' => return Ok(Token::Plus),
                '-' => return Ok(Token::Minus),
                '*' => return Ok(Token::Multiply),
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
                        "msg" => Ok(Token::Print),
                        "set" => Ok(Token::Set),
                        "if" => Ok(Token::If),
                        "else" => Ok(Token::Else),
                        _ => Ok(Token::Identifier(identifier)),
                    };
                }
                unknown => {
                    return Err(LexerError {
                        message: format!("Unexpected character: '{}'", unknown),
                        line: self.line,
                        column: self.column,
                    });
                }
            }
        }
        Err(LexerError {
            message: "End of input".to_string(),
            line: self.line,
            column: self.column,
        })
    }
}
