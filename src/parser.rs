// src/parser.rs

use crate::ast::{ASTNode, Expr, Value};
use crate::lexer::Token;
use crate::utils::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    // 借用競合を防ぐため、peek_token()の結果は cloned() する
    fn next_token(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    fn peek_token(&self) -> Option<Token> {
        self.tokens.get(self.position).cloned()
    }

    pub fn parse(&mut self) -> Result<ASTNode> {
        let mut statements = Vec::new();

        while let Some(token) = self.peek_token() {
            match token {
                Token::Package => {
                    self.next_token();
                    if let Some(Token::Identifier(name)) = self.next_token() {
                        statements.push(ASTNode::Package(name));
                    } else {
                        return Err(Error::Syntax("Expected package name".into()));
                    }
                }
                Token::Msg => {
                    self.next_token();
                    if let Some(Token::Text(msg)) = self.next_token() {
                        statements.push(ASTNode::Msg(msg));
                    } else {
                        return Err(Error::Syntax("Expected message string".into()));
                    }
                }
                Token::Exit => {
                    self.next_token();
                    statements.push(ASTNode::Exit);
                }
                Token::Identifier(name) => {
                    self.next_token(); // 消費する
                    if let Some(tok) = self.peek_token() {
                        match tok {
                            Token::Equals => {
                                self.next_token(); // '=' を消費
                                let expr = self.parse_expression()?;
                                // Variable は now (String, Box<Expr>) に変更している
                                statements.push(ASTNode::Variable(name, Box::new(expr)));
                            }
                            Token::LeftParen => {
                                self.next_token(); // '(' を消費
                                let args = self.parse_arguments()?;
                                statements.push(ASTNode::FunctionCall(name, args));
                            }
                            _ => {
                                return Err(Error::Syntax("Expected '=' or '(' after identifier".into()));
                            }
                        }
                    } else {
                        return Err(Error::Syntax("Unexpected end of input after identifier".into()));
                    }
                }
                _ => return Err(Error::Syntax(format!("Unexpected token: {:?}", token))),
            }
        }

        Ok(ASTNode::Program(statements))
    }

    // 簡易的な式解析
    fn parse_expression(&mut self) -> Result<Expr> {
        if let Some(token) = self.peek_token() {
            match token {
                Token::Number(n) => {
                    self.next_token();
                    Ok(Expr::Literal(Value::Number(n)))
                }
                Token::Text(s) => {
                    self.next_token();
                    Ok(Expr::Literal(Value::Text(s)))
                }
                Token::Identifier(name) => {
                    self.next_token();
                    Ok(Expr::Variable(name))
                }
                _ => Err(Error::Syntax(format!("Unexpected token in expression: {:?}", token))),
            }
        } else {
            Err(Error::Syntax("Unexpected end of input in expression".into()))
        }
    }

    // parse_arguments() は Vec<Expr> を返す
    fn parse_arguments(&mut self) -> Result<Vec<Expr>> {
        let mut args = Vec::new();
        while let Some(token) = self.peek_token() {
            match token {
                Token::RightParen => {
                    self.next_token(); // ')' を消費
                    break;
                }
                _ => {
                    let expr = self.parse_expression()?;
                    args.push(expr);
                    if let Some(Token::Comma) = self.peek_token() {
                        self.next_token(); // カンマを消費
                    }
                }
            }
        }
        Ok(args)
    }
}
