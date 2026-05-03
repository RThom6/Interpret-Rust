use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
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
}

impl Expr {
    pub fn evaluate(&self) -> Value {
        match self {
            Expr::Literal { value } => value.clone(),

            Expr::Grouping { expression } => expression.evaluate(),

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = left.evaluate();
                let right_val = right.evaluate();

                match (&operator.token_type, left_val, right_val) {
                    (TokenType::Plus, Value::Number(l), Value::Number(r)) => Value::Number(l + r),
                    (TokenType::Minus, Value::Number(l), Value::Number(r)) => Value::Number(l - r),
                    (TokenType::Multiply, Value::Number(l), Value::Number(r)) => {
                        Value::Number(l * r)
                    }
                    (TokenType::Divide, Value::Number(l), Value::Number(r)) => Value::Number(l / r),
                    _ => panic!("Runtime Error: Invalid operands for binary operator."),
                }
            }

            Expr::Unary { operator, right } => {
                let right_val = right.evaluate();

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
                let cond_val = condition.evaluate();
                match cond_val {
                    Value::Bool(true) => then_branch.evaluate(),
                    Value::Bool(false) => {
                        if let Some(else_expr) = else_branch {
                            else_expr.evaluate()
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
                let cond_val = condition.evaluate();

                match cond_val {
                    Value::Bool(true) => then_branch.evaluate(),
                    Value::Bool(false) => Value::Null,
                    _ => panic!("Runtime Error: While condition must be a boolean"),
                }
            }
        }
    }
}
