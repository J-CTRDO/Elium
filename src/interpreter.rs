// src/interpreter.rs

use crate::ast::{ASTNode, Expr, Value};
use crate::scope::Scope;
use crate::utils::error::{Error, Result};
use std::collections::HashMap;

// ※ここでは ast::Value を使用するので、独自の Value 定義は削除

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
    // 現在のスコープ（変数管理）
    pub scope: Scope,
    // 関数定義: 関数名 → (引数リスト, 関数本体（文のリスト）)
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
                // expr_box は Box<Expr> なので、*expr_box を &Expr に変換
                let value = self.evaluate_expression(&*expr_box)?;
                self.scope.set(name.clone(), value);
            }
            ASTNode::Msg(message) => {
                println!("{}", message);
            }
            ASTNode::If(condition_node, then_body, else_body) => {
                // 条件は ASTNode として格納されているので、仮に Literal または Variable として扱う
                let condition_expr = match &**condition_node {
                    ASTNode::Literal(val) => Expr::Literal(val.clone()),
                    ASTNode::Variable(name, _) => Expr::Variable(name.clone()),
                    _ => return Err(Error::Runtime("Unsupported condition expression".into())),
                };
                if let Value::Boolean(true) = self.evaluate_expression(&condition_expr)? {
                    // then_body は Vec<ASTNode> なので、clone() して渡す
                    self.interpret(then_body.clone())?;
                } else {
                    self.interpret(else_body.clone())?;
                }
            }
            ASTNode::Function(name, params, body) => {
                self.functions.insert(name.clone(), (params.clone(), body.clone()));
            }
            ASTNode::FunctionCall(name, args) => {
                // ここで self.functions.get(name) の borrow を回避するため、clone する
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
                // 新しいローカルスコープを作成
                let mut local_scope = Scope::new(Some(self.scope.clone()));
                for (param, arg) in params.iter().zip(args.iter()) {
                    let value = self.evaluate_expression(arg)?;
                    local_scope.set(param.clone(), value);
                }
                // 現在のスコープを退避して、ローカルスコープで関数本体を実行
                let previous_scope = self.scope.clone();
                self.scope = local_scope;
                self.interpret(body.clone())?;
                self.scope = previous_scope;
            }
            ASTNode::Exit => {
                println!("Exiting program.");
                std::process::exit(0);
            }
            _ => return Err(Error::Runtime(format!("Unexpected statement: {:?}", stmt))),
        }
        Ok(())
    }

    /// 式を評価する。Expr 型を受け取り、Value を返す
    fn evaluate_expression(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Literal(value) => Ok(value.clone()),
            Expr::Variable(name) => {
                self.scope
                    .get(name)
                    .ok_or_else(|| Error::Runtime(format!("Undefined variable: {}", name)))
            }
            Expr::BinaryOp(left, op, right) => {
                let left_value = self.evaluate_expression(left)?;
                let right_value = self.evaluate_expression(right)?;
                // borrowエラー回避のため、cloneしてエラーメッセージに使用
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
                Err(Error::Runtime("Function calls in expressions not supported".into()))
            }
        }
    }
}
