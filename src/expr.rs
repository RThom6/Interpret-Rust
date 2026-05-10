use std::collections::HashMap;

use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
}

pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.scopes.last_mut().unwrap().insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.get(name) {
                return Some(v.clone());
            }
        }
        None
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }
        Err(format!("Undefined variable '{}'.", name))
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal {
        value: Value,
    },

    Grouping {
        expression: Box<Expr>,
    },

    Unary {
        operator: Token,
        right: Box<Expr>,
    },

    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },

    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },

    While {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
    },

    Variable {
        name: Token,
    },

    Assign {
        name: Token,
        value: Box<Expr>,
    },

    Let {
        name: Token,
        initializer: Box<Expr>,
    },

    Block {
        exprs: Vec<Expr>,
    },
}

impl Expr {
    pub fn evaluate(&self, env: &mut Expr) -> Value {
        match self {
            Expr::Literal { value } => value.clone(),

            Expr::Grouping { expression } => expression.evaluate(env),

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = left.evaluate(env);
                let right_val = right.evaluate(env);

                match (&operator.token_type, left_val, right_val) {
                    (TokenType::Plus, Value::Number(l), Value::Number(r)) => Value::Number(l + r),
                    (TokenType::Plus, Value::String(l), Value::String(r)) => {
                        Value::String(format!("{}{}", l, r))
                    }
                    (TokenType::Minus, Value::Number(l), Value::Number(r)) => Value::Number(l - r),
                    (TokenType::Multiply, Value::Number(l), Value::Number(r)) => {
                        Value::Number(l * r)
                    }
                    (TokenType::Divide, Value::Number(l), Value::Number(r)) => Value::Number(l / r),
                    (TokenType::GreaterThan, Value::Number(l), Value::Number(r)) => {
                        Value::Bool(l > r)
                    }
                    (TokenType::LessThan, Value::Number(l), Value::Number(r)) => Value::Bool(l < r),
                    (TokenType::GreaterEqual, Value::Number(l), Value::Number(r)) => {
                        Value::Bool(l >= r)
                    }
                    (TokenType::LessEqual, Value::Number(l), Value::Number(r)) => {
                        Value::Bool(l <= r)
                    }
                    (TokenType::NotEqual, Value::Number(l), Value::Number(r)) => {
                        Value::Bool(l != r)
                    }
                    (TokenType::EqualEqual, Value::Number(l), Value::Number(r)) => {
                        Value::Bool(l == r)
                    }
                    _ => panic!("Runtime Error: Invalid operands for binary operator."),
                }
            }

            Expr::Unary { operator, right } => {
                let right_val = right.evaluate(env);

                match (&operator.token_type, right_val) {
                    (TokenType::Minus, Value::Number(n)) => Value::Number(-n),
                    (TokenType::Bang, Value::Bool(r)) => Value::Bool(!r),
                    _ => panic!("Runtime Error: Invalid operand for unary operator."),
                }
            }

            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_val = condition.evaluate(env);
                match cond_val {
                    Value::Bool(true) => then_branch.evaluate(env),
                    Value::Bool(false) => {
                        if let Some(else_expr) = else_branch {
                            else_expr.evaluate(env)
                        } else {
                            Value::Null
                        }
                    }
                    _ => panic!("Runtime Error: If condition must be a boolean."),
                }
            }

            Expr::While {
                condition,
                then_branch,
            } => {
                loop {
                    match condition.evaluate(env) {
                        Value::Bool(true) => {
                            then_branch.evaluate(env);
                        }
                        Value::Bool(false) => break,
                        _ => panic!("Runtime Error: While condition must be a boolean"),
                    }
                }
                Value::Null
            }

            _ => Value::Null,
        }
    }
}
