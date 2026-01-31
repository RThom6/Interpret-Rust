use crate::token::Token;

pub enum Stmnt {
    Expr(Expr),
    Let(Name, Expr),
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, CLone)]
pub struct Unary {}

#[derive(Debug, CLone)]
pub struct Literal {}

#[derive(Debug, CLone)]
pub struct Grouping {}

pub enum Expr {
    Binary(BinaryExpr),
    Literal(Literal),
}

mod visit {}
