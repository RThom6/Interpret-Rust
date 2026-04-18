use crate::expr::{Expr, Value};
use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) {}
}

impl Parser {
    fn expression(&mut self) -> Expr {
        return self.equality();
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        while (self.check_match(&[TokenKind::NotEqual, TokenKind::EqualEqual])) {
            let operator: Token = self.previous();
            let right: Expr = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();

        while self.check_match(&[
            TokenKind::GreaterThan,
            TokenKind::GreaterEqual,
            TokenKind::LessThan,
            TokenKind::LessEqual,
        ]) {
            let operator: Token = self.previous();
            let right: Expr = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();

        while self.check_match(&[TokenKind::Minus, TokenKind::Plus]) {
            let operator: Token = self.previous();
            let right: Expr = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();

        while self.check_match(&[TokenKind::Divide, TokenKind::Multiply]) {
            let operator: Token = self.previous();
            let right: Expr = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.check_match(&[TokenKind::Minus, TokenKind::Bang]) {
            let operator: Token = self.previous();
            let right: Expr = self.primary();
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.check_match(&[TokenKind::Identifier]) {
            let value: String = self.previous().lexeme.parse().unwrap();
            return Expr::Literal {
                value: Value::String(value),
            };
        }

        if self.check_match(&[TokenKind::Number]) {
            let value: f64 = self.previous().lexeme.parse().unwrap();
            return Expr::Literal {
                value: Value::Number(value),
            };
        }

        if self.check_match(&[TokenKind::StringLiteral]) {
            let value: String = self.previous().lexeme.parse().unwrap();
            return Expr::Literal {
                value: Value::String(value),
            };
        }

        if self.check_match(&[TokenKind::ParenthesesLeft]) {
            let expr: Expr = self.expression();
            self.consume(
                TokenKind::ParenthesesRight,
                "Expected ')' after expression.",
            );
            return Expr::Grouping {
                expression: Box::new(expr),
            };
        }

        panic!("Unexpected token: {:?}", self.peek());
    }

    fn consume(&mut self, kind: TokenKind, message: &str) {
        if self.check(kind.clone()) {
            self.advance();
            return;
        }

        panic!("{}", message);
    }

    fn parse_error(&mut self, token: Token, message: &str) {}

    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn check_match(&mut self, types: &[TokenKind]) -> bool {
        for t in types {
            if self.check(t.clone()) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.tokens[self.current].token_type.kind() == kind
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}
