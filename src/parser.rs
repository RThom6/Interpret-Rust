use crate::error::Error;
use crate::expr::{Expr, Value};
use crate::token::{Token, TokenKind, TokenType};

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Expr {
        let mut stmts: Vec<Expr> = Vec::new();
        while !self.is_at_end() {
            stmts.push(self.statement());
        }
        Expr::Block { exprs: stmts }
    }
}

impl<'a> Parser<'a> {
    fn statement(&mut self) -> Expr {
        match self.peek().token_type.kind() {
            TokenKind::If => self.if_statement(),
            TokenKind::While => self.while_statement(),
            TokenKind::Let => self.let_statement(),
            TokenKind::BraceLeft => self.block(),
            _ => self.expression_statement(),
        }
    }

    fn block(&mut self) -> Expr {
        match self.consume(TokenKind::BraceLeft, "Expected '{'") {
            Ok(_) => {}
            Err(err) => panic!("{}", err.message),
        }

        let mut exprs: Vec<Expr> = Vec::new();
        while !self.check(TokenKind::BraceRight) && !self.is_at_end() {
            exprs.push(self.statement());
        }

        match self.consume(TokenKind::BraceRight, "Expected '}'") {
            Ok(_) => {}
            Err(err) => panic!("{}", err.message),
        }

        Expr::Block { exprs }
    }

    fn let_statement(&mut self) -> Expr {
        match self.consume(TokenKind::Let, "Expected 'let'") {
            Ok(_) => {}
            Err(err) => panic!("{}", err.message),
        }

        let name = match self.consume(TokenKind::Identifier, "Expected identifier after 'let'") {
            Ok(t) => t,
            Err(err) => panic!("{}", err.message),
        };

        match self.consume(TokenKind::Equal, "Expected '=' after let name") {
            Ok(_) => {}
            Err(err) => panic!("{}", err.message),
        }

        let initializer = self.expression();

        match self.consume(TokenKind::SemiColon, "Expected ';' after let declaration") {
            Ok(_) => {}
            Err(err) => panic!("{}", err.message),
        }

        Expr::Let {
            name,
            initializer: Box::new(initializer),
        }
    }

    fn expression_statement(&mut self) -> Expr {
        let expr = self.expression();
        match self.consume(TokenKind::SemiColon, "Expected ';' after expression") {
            Ok(_) => {}
            Err(err) => panic!("{}", err.message),
        }
        expr
    }

    fn if_statement(&mut self) -> Expr {
        match self.consume(TokenKind::If, "Expected 'If' statement") {
            Ok(_) => {}
            Err(err) => panic!("{}", err.message),
        }

        match self.consume(TokenKind::ParenthesesLeft, "Expected opening parentheses") {
            Ok(_) => {}
            Err(err) => panic!("{}", err.message),
        }

        let condition = self.expression();

        match self.consume(TokenKind::ParenthesesRight, "Expected closing parentheses") {
            Ok(_) => {}
            Err(err) => panic!("{}", err.message),
        }

        let then_branch = self.statement();

        let else_branch = if self.check_match(&[TokenKind::Else]) {
            Some(Box::new(self.statement()))
        } else {
            None
        };

        Expr::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch,
        }
    }

    fn while_statement(&mut self) -> Expr {
        match self.consume(TokenKind::While, "Expected 'While' statement") {
            Ok(_) => {}
            Err(err) => panic!("{}", err.message),
        }

        match self.consume(TokenKind::ParenthesesLeft, "Expected opening parentheses") {
            Ok(_) => {}
            Err(err) => panic!("{}", err.message),
        }

        let condition = self.expression();

        match self.consume(TokenKind::ParenthesesRight, "Expected closing parentheses") {
            Ok(_) => {}
            Err(err) => panic!("{}", err.message),
        }

        let then_branch = self.statement();

        Expr::While {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
        }
    }

    fn expression(&mut self) -> Expr {
        self.assignment()
    }

    fn assignment(&mut self) -> Expr {
        let expr = self.equality();

        if self.check_match(&[TokenKind::Equal]) {
            let value = self.assignment();
            match expr {
                Expr::Variable { name } => {
                    return Expr::Assign {
                        name,
                        value: Box::new(value),
                    };
                }
                _ => panic!("Invalid assignment target."),
            }
        }

        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        while self.check_match(&[TokenKind::NotEqual, TokenKind::EqualEqual]) {
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
        if self.check_match(&[TokenKind::True]) {
            return Expr::Literal {
                value: Value::Bool(true),
            };
        }

        if self.check_match(&[TokenKind::False]) {
            return Expr::Literal {
                value: Value::Bool(false),
            };
        }

        if self.check_match(&[TokenKind::Null]) {
            return Expr::Literal { value: Value::Null };
        }

        if self.check_match(&[TokenKind::Identifier]) {
            let name = self.previous();
            return Expr::Variable { name };
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
            match self.consume(
                TokenKind::ParenthesesRight,
                "Expected ')' after expression.",
            ) {
                Ok(token) => token,
                Err(e) => panic!("{}", e.message), // Placeholder error, wnat to hand it back have it store it to print as a trace at theend maybe? Decision pending i guess
            };
            return Expr::Grouping {
                expression: Box::new(expr),
            };
        }

        panic!("Unexpected token: {:?}", self.peek());
    }

    // TODO: error handling method, looks for statement boundary, should call it when I catch a parse error - Find in chapter 6
    // fn synchronize(&mut self) {
    //     self.advance();

    //     while !self.is_at_end() {
    //         if self.previous().token_type == TokenType::SemiColon {
    //             return;
    //         }

    //         match self.peek().token_type.kind() {
    //             TokenKind::Return => {}
    //             _ => {
    //                 self.advance();
    //             }
    //         }
    //     }
    // }

    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<Token, Error> {
        if self.check(kind.clone()) {
            return Ok(self.advance());
        }
        // TODO: implement error handling properly, will probably do it when it's mostly done or debugging starts annoying me
        // Need to report the error to the user and then pass it back and handle it accordingly
        // Probably looking to report multiple errors at once to be useful since you don't want to just debug
        // based on  the first error of your code and nothing else, would get very annoying very quickly.
        return Err(self.parse_error(kind, message));
    }

    fn parse_error(&mut self, kind: TokenKind, message: &str) -> Error {
        // error handling
        return Error::new("placeholder error error", 765);
    }

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
