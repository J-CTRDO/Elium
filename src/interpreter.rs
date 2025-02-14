use crate::ast::{ASTNode, Expr};
use crate::utils::error::{Error, Result};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    Text(String),
    Boolean(bool),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Text(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Interpreter {
    variables: HashMap<String, Value>,  // 変数を保持
    functions: HashMap<String, ASTNode>, // 関数を保持
}

impl Interpreter {
    /// 新しいインタプリタを作成
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
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
                self.variables.insert(name.clone(), value);
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
            ASTNode::Function(name, _params, body) => {
                self.functions.insert(name.clone(), ASTNode::Function(name.clone(), _params.clone(), body.clone()));
            }
            ASTNode::FunctionCall(name, args) => {
                if let Some(ASTNode::Function(_name, params, body)) = self.functions.get(name) {
                    if params.len() != args.len() {
                        return Err(Error::Runtime(format!(
                            "Function {} expected {} arguments, but got {}",
                            name,
                            params.len(),
                            args.len()
                        )));
                    }
                    let mut local_variables = HashMap::new();
                    for (param, arg) in params.iter().zip(args.iter()) {
                        local_variables.insert(param.clone(), self.evaluate_expression(arg)?);
                    }
                    let mut sub_interpreter = Self {
                        variables: local_variables,
                        functions: self.functions.clone(),
                    };
                    sub_interpreter.interpret(body.clone())?;
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
                .variables
                .get(name)
                .cloned()
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
