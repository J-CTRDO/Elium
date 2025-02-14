use crate::ast::{ASTNode, Expr};
use crate::scope::Scope;
use crate::utils::error::{Error, Result};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    Text(String),
    Boolean(bool),
    None, // 戻り値がない場合に対応
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Text(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Interpreter {
    scope: Scope, // スコープ管理
    functions: HashMap<String, (Vec<String>, Vec<ASTNode>)>, // 関数名 -> (引数, 関数本体)
}

impl Interpreter {
    /// 新しいインタプリタを作成
    pub fn new() -> Self {
        Self {
            scope: Scope::new(None), // グローバルスコープの初期化
            functions: HashMap::new(),
        }
    }

    /// ASTを解釈・実行
    pub fn interpret(&mut self, stmts: Vec<ASTNode>) -> Result<()> {
        for stmt in stmts {
            self.execute_statement(&stmt)?;
        }
        Ok(())
    }

    /// ステートメントを実行
    fn execute_statement(&mut self, stmt: &ASTNode) -> Result<()> {
        match stmt {
            ASTNode::Variable(name, expr) => {
                let value = self.evaluate_expression(expr)?;
                self.scope.set(name.clone(), value); // スコープに変数を登録
            }
            ASTNode::Msg(message) => {
                println!("{}", message);
            }
            ASTNode::If(condition, then_body, else_body) => {
                if let Value::Boolean(true) = self.evaluate_expression(condition)? {
                    self.interpret(then_body.clone())?;
                } else if let Some(else_body) = else_body {
                    self.interpret(else_body.clone())?;
                }
            }
            ASTNode::Function(name, params, body) => {
                self.functions.insert(name.clone(), (params.clone(), body.clone())); // 関数を登録
            }
            ASTNode::FunctionCall(name, args) => {
                if let Some((params, body)) = self.functions.get(name) {
                    if params.len() != args.len() {
                        return Err(Error::Runtime(format!(
                            "Function {} expected {} arguments, but got {}",
                            name,
                            params.len(),
                            args.len()
                        )));
                    }

                    // 関数呼び出し用のローカルスコープを生成
                    let mut local_scope = Scope::new(Some(self.scope.clone()));
                    for (param, arg) in params.iter().zip(args.iter()) {
                        let value = self.evaluate_expression(arg)?;
                        local_scope.set(param.clone(), value);
                    }

                    // 現在のスコープをローカルスコープに切り替えて実行
                    let previous_scope = self.scope.clone();
                    self.scope = local_scope;
                    self.interpret(body.clone())?;
                    self.scope = previous_scope; // 元のスコープに戻す
                } else {
                    return Err(Error::Runtime(format!("Function {} not found", name)));
                }
            }
            ASTNode::Exit => {
                println!("Exiting program.");
                std::process::exit(0);
            }
            _ => return Err(Error::Runtime(format!("Unexpected statement: {:?}", stmt))),
        }
        Ok(())
    }

    /// 式を評価
    fn evaluate_expression(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Literal(value) => Ok(value.clone()),
            Expr::Variable(name) => self
                .scope
                .get(name)
                .ok_or_else(|| Error::Runtime(format!("Undefined variable: {}", name))),
            Expr::Binary(left, op, right) => {
                let left_value = self.evaluate_expression(left)?;
                let right_value = self.evaluate_expression(right)?;
                match (left_value, right_value, op.as_str()) {
                    (Value::Number(l), Value::Number(r), "+") => Ok(Value::Number(l + r)),
                    (Value::Number(l), Value::Number(r), "-") => Ok(Value::Number(l - r)),
                    (Value::Number(l), Value::Number(r), "*") => Ok(Value::Number(l * r)),
                    (Value::Number(l), Value::Number(r), "/") => {
                        if r == 0 {
                            Err(Error::Runtime("Division by zero".into()))
                        } else {
                            Ok(Value::Number(l / r))
                        }
                    }
                    (Value::Text(l), Value::Text(r), "+") => Ok(Value::Text(l + &r)),
                    (Value::Number(l), Value::Number(r), ">") => Ok(Value::Boolean(l > r)),
                    (Value::Number(l), Value::Number(r), "<") => Ok(Value::Boolean(l < r)),
                    (Value::Number(l), Value::Number(r), "==") => Ok(Value::Boolean(l == r)),
                    _ => Err(Error::Runtime(format!(
                        "Invalid operation for {:?} and {:?} with operator {}",
                        left_value, right_value, op
                    ))),
                }
            }
            Expr::Input(prompt) => {
                println!("{}", prompt);
                let mut buffer = String::new();
                std::io::stdin()
                    .read_line(&mut buffer)
                    .map_err(|_| Error::Runtime("Failed to read input".into()))?;
                let input = buffer.trim().to_string();
                if let Ok(number) = input.parse::<i64>() {
                    Ok(Value::Number(number))
                } else {
                    Ok(Value::Text(input))
                }
            }
        }
    }
}
