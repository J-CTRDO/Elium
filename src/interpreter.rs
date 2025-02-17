// src/interpreter.rs

use crate::ast::{ASTNode, Expr, Value};
use crate::scope::Scope;
use crate::utils::error::{Error, Result};
use std::collections::HashMap;

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Text(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Array(arr) => write!(f, "{:?}", arr),
            Value::Map(map) => write!(f, "{:?}", map),
            Value::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Interpreter {
    // 変数や関数定義のためのスコープ
    pub scope: Scope,
    // 関数定義：関数名 → (引数リスト, 関数本体の文リスト)
    pub functions: HashMap<String, (Vec<String>, Vec<ASTNode>)>,
}

impl Interpreter {
    /// 新しいインタプリタを作成
    pub fn new() -> Self {
        Self {
            scope: Scope::new(None),
            functions: HashMap::new(),
        }
    }

    /// AST（文のリスト）を実行する
    pub fn interpret(&mut self, stmts: Vec<ASTNode>) -> Result<()> {
        for stmt in stmts {
            self.execute_statement(&stmt)?;
        }
        Ok(())
    }

    /// 各文を実行する
    fn execute_statement(&mut self, stmt: &ASTNode) -> Result<()> {
        match stmt {
            ASTNode::Variable(name, expr_box) => {
                // expr_box は Box<Expr> なので、*expr_box で解包
                let value = self.evaluate_expression(&*expr_box)?;
                self.scope.set(name.clone(), value);
            }
            ASTNode::Msg(message) => {
                println!("{}", message);
            }
            ASTNode::If(condition_node, then_body, else_body) => {
                // 条件部分は、ASTNode::Literal または ASTNode::Variable として想定
                let condition_expr = match &**condition_node {
                    ASTNode::Literal(val) => Expr::Literal(val.clone()),
                    ASTNode::Variable(name, _) => Expr::Variable(name.clone()),
                    _ => return Err(Error::Runtime("Unsupported condition expression".into())),
                };
                if let Value::Boolean(true) = self.evaluate_expression(&condition_expr)? {
                    self.interpret(then_body.clone())?;
                } else {
                    self.interpret(else_body.clone())?;
                }
            }
            ASTNode::Function(name, params, body) => {
                self.functions.insert(name.clone(), (params.clone(), body.clone()));
            }
            ASTNode::FunctionCall(name, args) => {
                // 関数呼び出し
                let (params, body) = self.functions.get(name)
                    .cloned()
                    .ok_or_else(|| Error::Runtime(format!("Function {} not found", name)))?;
                if params.len() != args.len() {
                    return Err(Error::Runtime(format!(
                        "Function {} expected {} arguments, but got {}",
                        name,
                        params.len(),
                        args.len()
                    )));
                }
                // ローカルスコープを作成して引数を評価
                let mut local_scope = Scope::new(Some(self.scope.clone()));
                for (param, arg) in params.iter().zip(args.iter()) {
                    let value = self.evaluate_expression(arg)?;
                    local_scope.set(param.clone(), value);
                }
                let previous_scope = self.scope.clone();
                self.scope = local_scope;
                self.interpret(body.clone())?;
                self.scope = previous_scope;
            }
            ASTNode::Exit => {
                println!("Exiting program.");
                std::process::exit(0);
            }
            // まだ未実装のバリアントはエラーにする
            _ => return Err(Error::Runtime(format!("Unexpected statement: {:?}", stmt))),
        }
        Ok(())
    }

    /// 式 (Expr) を評価して Value を返す
    fn evaluate_expression(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Literal(value) => Ok(value.clone()),
            Expr::Variable(name) => {
                self.scope.get(name)
                    .ok_or_else(|| Error::Runtime(format!("Undefined variable: {}", name)))
            }
            Expr::BinaryOp(left, op, right) => {
                let left_value = self.evaluate_expression(left)?;
                let right_value = self.evaluate_expression(right)?;
                // 値の所有権の問題を避けるため、必要に応じて clone する
                match (left_value.clone(), right_value.clone(), op.as_str()) {
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
                if let Ok(n) = input.parse::<i64>() {
                    Ok(Value::Number(n))
                } else {
                    Ok(Value::Text(input))
                }
            }
            Expr::FunctionCall(_name, _args) => {
                // 式内の関数呼び出しはここでは未実装（または後で実装する）
                Err(Error::Runtime("Function calls in expressions not supported".into()))
            }
        }
    }
}
