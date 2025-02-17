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

    // 次のトークンを消費して返す（所有権を持つ）
    fn next_token(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    // 次のトークンを参照する（クローンして返す）
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
                Token::Import => {
                    self.next_token();
                    // 例: Import from elium to os
                    if let Some(Token::From) = self.next_token() {
                        if let Some(Token::Identifier(pkg)) = self.next_token() {
                            if let Some(Token::To) = self.next_token() {
                                if let Some(Token::Identifier(target)) = self.next_token() {
                                    statements.push(ASTNode::Import(pkg, Some(target)));
                                } else {
                                    return Err(Error::Syntax("Expected target package for import".into()));
                                }
                            } else {
                                // "from" のみの場合
                                statements.push(ASTNode::Import(pkg, None));
                            }
                        } else {
                            return Err(Error::Syntax("Expected package name after 'from'".into()));
                        }
                    } else {
                        return Err(Error::Syntax("Expected 'from' in import statement".into()));
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
                Token::If => {
                    // if 文: if ( condition ) { then } else { else }
                    self.next_token(); // consume 'if'
                    // 期待: '(' condition ')'
                    if let Some(Token::LeftParen) = self.next_token() {
                        let condition = self.parse_expression()?;
                        if let Some(Token::RightParen) = self.next_token() {
                            // 期待: '{' then statements '}'
                            if let Some(Token::LeftBrace) = self.next_token() {
                                let then_body = self.parse_block()?;
                                // 期待: '}' already consumed by parse_block
                                // else 部分は任意
                                let else_body = if let Some(Token::Else) = self.peek_token() {
                                    self.next_token(); // consume 'else'
                                    if let Some(Token::LeftBrace) = self.next_token() {
                                        self.parse_block()?
                                    } else {
                                        return Err(Error::Syntax("Expected '{' after else".into()));
                                    }
                                } else {
                                    Vec::new()
                                };
                                // condition を Expr から ASTNode::Literal を介して表現
                                let cond_node = ASTNode::Literal(match condition {
                                    Expr::Literal(v) => v,
                                    _ => return Err(Error::Syntax("Complex condition expressions not supported yet".into())),
                                });
                                statements.push(ASTNode::If(Box::new(cond_node), then_body, else_body));
                            } else {
                                return Err(Error::Syntax("Expected '{' after if condition".into()));
                            }
                        } else {
                            return Err(Error::Syntax("Expected ')' after if condition".into()));
                        }
                    } else {
                        return Err(Error::Syntax("Expected '(' after if".into()));
                    }
                }
                Token::Function => {
                    // function 定義: function ( name=add, a, b ) { ... } return expr
                    self.next_token(); // consume 'function'
                    if let Some(Token::LeftParen) = self.next_token() {
                        // parse parameter list
                        let mut params = Vec::new();
                        let mut func_name = String::new();
                        while let Some(token) = self.peek_token() {
                            match token {
                                Token::RightParen => { self.next_token(); break; }
                                Token::Comma => { self.next_token(); continue; }
                                Token::Identifier(_) => {
                                    let id = self.next_token().unwrap(); // Identifier
                                    if let Token::Identifier(name) = id {
                                        // もしパラメーターの中で "name=add" のような記述があれば、
                                        // それを関数名として採用
                                        if let Some(Token::Equals) = self.peek_token() {
                                            self.next_token(); // consume '='
                                            if let Some(Token::Identifier(n)) = self.next_token() {
                                                func_name = n;
                                            } else {
                                                return Err(Error::Syntax("Expected function name after '='".into()));
                                            }
                                        } else {
                                            params.push(name);
                                        }
                                    }
                                }
                                _ => return Err(Error::Syntax(format!("Unexpected token in function parameters: {:?}", token))),
                            }
                        }
                        // 期待: '{'
                        if let Some(Token::LeftBrace) = self.next_token() {
                            let body = self.parse_block()?;
                            // 期待: optional return statement at the end of the block is parsed as part of the body
                            statements.push(ASTNode::Function(func_name, params, body));
                        } else {
                            return Err(Error::Syntax("Expected '{' to start function body".into()));
                        }
                    } else {
                        return Err(Error::Syntax("Expected '(' after function".into()));
                    }
                }
                Token::Identifier(name) => {
                    // 変数代入または関数呼び出し
                    let id = self.next_token().unwrap(); // consume identifier
                    if let Some(token) = self.peek_token() {
                        match token {
                            Token::Equals => {
                                self.next_token(); // consume '='
                                let expr = self.parse_expression()?;
                                statements.push(ASTNode::Variable(name, Box::new(expr)));
                            }
                            Token::LeftParen => {
                                self.next_token(); // consume '('
                                let args = self.parse_expression_list()?; // parse comma-separated expressions
                                // 期待: ')'
                                if let Some(Token::RightParen) = self.next_token() {
                                    statements.push(ASTNode::FunctionCall(name, args));
                                } else {
                                    return Err(Error::Syntax("Expected ')' after function call arguments".into()));
                                }
                            }
                            _ => {
                                return Err(Error::Syntax("Expected '=' for variable assignment or '(' for function call after identifier".into()));
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

    // 簡易的な式解析：二項演算は左結合とする（加減乗算のみ対応）
    fn parse_expression(&mut self) -> Result<Expr> {
        let mut expr = self.parse_primary()?;
        while let Some(token) = self.peek_token() {
            match token {
                Token::Plus | Token::Minus | Token::Multiply => {
                    let op = match token {
                        Token::Plus => "+".to_string(),
                        Token::Minus => "-".to_string(),
                        Token::Multiply => "*".to_string(),
                        _ => unreachable!(),
                    };
                    self.next_token(); // consume operator
                    let right = self.parse_primary()?;
                    expr = Expr::BinaryOp(Box::new(expr), op, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    // parse_primary: 基本的な式の解析
    fn parse_primary(&mut self) -> Result<Expr> {
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
                    let ident = name.clone();
                    self.next_token(); // consume identifier
                    // 関数呼び出しの場合、後ろに '(' が続く
                    if let Some(Token::LeftParen) = self.peek_token() {
                        self.next_token(); // consume '('
                        let args = self.parse_expression_list()?;
                        if let Some(Token::RightParen) = self.next_token() {
                            Ok(Expr::FunctionCall(ident, args))
                        } else {
                            Err(Error::Syntax("Expected ')' after function call arguments".into()))
                        }
                    } else {
                        Ok(Expr::Variable(ident))
                    }
                }
                _ => Err(Error::Syntax(format!("Unexpected token in expression: {:?}", token))),
            }
        } else {
            Err(Error::Syntax("Unexpected end of input in expression".into()))
        }
    }

    // parse_expression_list: カンマ区切りの式リストを解析して Vec<Expr> を返す
    fn parse_expression_list(&mut self) -> Result<Vec<Expr>> {
        let mut args = Vec::new();
        loop {
            if let Some(token) = self.peek_token() {
                if token == Token::RightParen {
                    break;
                }
            } else {
                break;
            }
            let expr = self.parse_expression()?;
            args.push(expr);
            if let Some(token) = self.peek_token() {
                if token == Token::Comma {
                    self.next_token(); // consume comma
                    continue;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(args)
    }

    // parse_block: '{' ... '}' の中の文を解析する
    fn parse_block(&mut self) -> Result<Vec<ASTNode>> {
        let mut stmts = Vec::new();
        while let Some(token) = self.peek_token() {
            if token == Token::RightBrace {
                self.next_token(); // consume '}'
                break;
            }
            // 文ごとに parse_expression() やその他の文解析を呼び出す
            // ここでは簡単のため、変数代入、msg などを parse() の処理と同様に行う
            match token {
                Token::Identifier(name) => {
                    let id = self.next_token().unwrap(); // consume identifier
                    if let Some(tok) = self.peek_token() {
                        match tok {
                            Token::Equals => {
                                self.next_token(); // consume '='
                                let expr = self.parse_expression()?;
                                stmts.push(ASTNode::Variable(name.clone(), Box::new(expr)));
                            }
                            Token::LeftParen => {
                                self.next_token(); // consume '('
                                let args = self.parse_expression_list()?;
                                if let Some(Token::RightParen) = self.next_token() {
                                    stmts.push(ASTNode::FunctionCall(name.clone(), args));
                                } else {
                                    return Err(Error::Syntax("Expected ')' in function call".into()));
                                }
                            }
                            _ => return Err(Error::Syntax("Unexpected token in block after identifier".into())),
                        }
                    } else {
                        return Err(Error::Syntax("Unexpected end of input in block".into()));
                    }
                }
                Token::Msg => {
                    self.next_token();
                    if let Some(Token::Text(msg)) = self.next_token() {
                        stmts.push(ASTNode::Msg(msg));
                    } else {
                        return Err(Error::Syntax("Expected message string in block".into()));
                    }
                }
                _ => return Err(Error::Syntax(format!("Unexpected token in block: {:?}", token))),
            }
        }
        Ok(stmts)
    }
}
