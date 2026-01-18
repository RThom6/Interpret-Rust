use std::process::exit;

use crate::error::{Error, report};
use crate::token::{KeywordType, Token, TokenType};

#[allow(dead_code)]
static KEYWORDS: [&str; 8] = [
    "if", "else", "while", "for", "return", "break", "continue", "null",
];

pub struct TokenScanner {
    input: String,
    tokens: Vec<Token>,
    pos: usize,
    line: usize,
}

impl TokenScanner {
    pub fn new(input: String) -> Self {
        let mut scanner = TokenScanner {
            input,
            tokens: Vec::new(),
            pos: 0,
            line: 0,
        };
        scanner.populate_tokens();
        scanner
    }

    pub fn next_token(&mut self) -> Option<&TokenType> {
        if self.pos >= self.tokens.len() {
            None
        } else {
            let t = &self.tokens[self.pos];
            self.pos += 1;
            Some(t)
        }
    }

    pub fn peek_token(&self) -> Option<&TokenType> {
        self.tokens.get(self.pos)
    }
}

impl TokenScanner {
    fn populate_tokens(&mut self) {
        let mut chars = self.input.char_indices().peekable();

        // Does this fall off gracefully at EOF?
        while let Some((_, ch)) = chars.peek().cloned() {
            match ch {
                ' ' | '\t' | '\r' => {
                    chars.next();
                }
                '\n' => {
                    // don't terminate expression by new line
                    self.line += 1;
                    chars.next();
                }
                '+' => {
                    chars.next();
                    self.tokens
                        .push(Token::new(TokenType::Plus, "+", self.line.clone()));
                }
                '-' => {
                    chars.next();
                    self.tokens
                        .push(Token::new(TokenType::Minus, "-", self.line.clone()));
                }
                '=' => {
                    chars.next();
                    if Self::match_expected(&mut chars, '=') {
                        self.tokens.push(Token::new(
                            TokenType::EqualEqual,
                            "==",
                            self.line.clone(),
                        ));
                    } else {
                        self.tokens
                            .push(Token::new(TokenType::Equal, "=", self.line.clone()));
                    }
                }
                '>' => {
                    chars.next();
                    if Self::match_expected(&mut chars, '=') {
                        self.tokens.push(Token::new(
                            TokenType::GreaterEqual,
                            ">=",
                            self.line.clone(),
                        ));
                    } else {
                        self.tokens.push(Token::new(
                            TokenType::GreaterThan,
                            ">",
                            self.line.clone(),
                        ));
                    }
                }
                '<' => {
                    chars.next();
                    if Self::match_expected(&mut chars, '=') {
                        self.tokens
                            .push(Token::new(TokenType::LessEqual, "<=", self.line.clone()));
                    } else {
                        self.tokens
                            .push(Token::new(TokenType::LessThan, "<", self.line.clone()));
                    }
                }
                '!' => {
                    chars.next();
                    if Self::match_expected(&mut chars, '=') {
                        self.tokens
                            .push(Token::new(TokenType::NotEqual, "!=", self.line.clone()));
                    } else {
                        self.tokens.push(Token::new(
                            TokenType::Invalid("Unexpected '!'".into()),
                            "Temp error thingy",
                            self.line.clone(),
                        ));
                    }
                }
                '{' => {
                    chars.next();
                    self.tokens
                        .push(Token::new(TokenType::BraceLeft, "{", self.line.clone()));
                }
                '}' => {
                    chars.next();
                    self.tokens
                        .push(Token::new(TokenType::BraceRight, "}", self.line.clone()));
                }
                '(' => {
                    chars.next();
                    self.tokens
                        .push(Token::new(TokenType::BracketLeft, "(", self.line.clone()));
                }
                ')' => {
                    chars.next();
                    self.tokens
                        .push(Token::new(TokenType::BracketRight, ")", self.line.clone()));
                }
                '[' => {
                    chars.next();
                    self.tokens.push(Token::new(
                        TokenType::ParenthesesLeft,
                        "[",
                        self.line.clone(),
                    ));
                }
                ']' => {
                    chars.next();
                    self.tokens.push(Token::new(
                        TokenType::ParenthesesRight,
                        ")",
                        self.line.clone(),
                    ));
                }
                '"' => {
                    chars.next();
                    let mut s = String::new();

                    while let Some((_, c)) = chars.next() {
                        if c == '"' {
                            break;
                        }
                        s.push(c);
                    }

                    if chars.peek().is_none() {
                        self.tokens.push(Token::new(
                            TokenType::Invalid("Unterminated string literal".into()),
                            "Handle this error in a bit",
                            self.line.clone(),
                        ));
                    } else {
                        self.tokens.push(Token::new(
                            TokenType::StringLiteral(s),
                            &s.clone(),
                            self.line.clone(),
                        ));
                    }
                }
                '\'' => {
                    chars.next();

                    let char_value = match chars.next() {
                        Some((_, c)) => c,
                        None => {
                            self.tokens.push(Token::new(
                                TokenType::Invalid("Unterminated character literal".into()),
                                "other error again",
                                self.line.clone(),
                            ));
                            continue;
                        }
                    };

                    match chars.next() {
                        Some((_, '\'')) => {
                            self.tokens.push(Token::new(
                                TokenType::CharLiteral(char_value),
                                &ch.clone().to_string().as_str(),
                                self.line.clone(),
                            ));
                        }
                        _ => {
                            self.tokens.push(Token::new(
                                TokenType::Invalid("Unterminated character literal".into()),
                                &ch.clone().to_string().as_str(),
                                self.line.clone(),
                            ));
                        }
                    }
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut s = String::new();
                    s.push(ch);
                    chars.next();

                    while let Some((_, c)) = chars.peek() {
                        if c.is_alphanumeric() || *c == '_' {
                            s.push(*c);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    if KEYWORDS.contains(&s.as_str()) {
                        self.tokens.push(Token::new(
                            TokenType::Keyword(KeywordType::from_str(&s).unwrap()),
                            &s.as_str(),
                            self.line.clone(),
                        )); // TODO: Error handling
                    } else {
                        self.tokens.push(Token::new(
                            TokenType::Identifier(s),
                            &s.as_str(),
                            self.line.clone(),
                        ));
                    }
                }
                '/' => {
                    if let Some((_, '/')) = chars.peek() {
                        chars.next();
                        while let Some((_, c)) = chars.peek() {
                            if *c == '\n' {
                                // Comments go until the end of a line
                                // TODO: Add eof? We'd get stuck without it or reach an error state. Should look at this in multiple locations
                                break;
                            }
                            chars.next();
                        }
                    } else if let Some((_, '*')) = chars.peek() {
                        chars.next();
                        while let Some((_, c)) = chars.peek() {
                            if *c == '*' {
                                chars.next();
                                if let Some((_, '/')) = chars.peek() {
                                    chars.next();
                                    break;
                                }
                            }
                            chars.next();
                        }
                    } else {
                        self.tokens
                            .push(Token::new(TokenType::Divide, "/", self.line.clone()));
                    }
                }
                '*' => {
                    self.tokens
                        .push(Token::new(TokenType::Multiply, "*", self.line.clone()));
                }
                '0'..='9' => {
                    let mut s = "".to_owned();
                    s.push(ch);

                    while let Some((_, c)) = chars.peek() {
                        if !c.is_digit(10) {
                            self.tokens.push(Token::new(
                                TokenType::Number(s.parse::<u32>().unwrap()),
                                s.as_str(),
                                self.line.clone(),
                            ));
                            break;
                        }

                        s.push(ch);
                        chars.next();
                    }
                }
                ';' => self
                    .tokens
                    .push(Token::new(TokenType::SemiColon, ";", self.line.clone())),
                _ => {
                    report(Error::new("Invalid Character Placeholder", 16));
                    exit(1);
                }
            }
        }
    }

    /// Checks whether the next character matches the expected character.
    fn match_expected<I: Iterator<Item = (usize, char)>>(
        chars: &mut std::iter::Peekable<I>,
        expected: char,
    ) -> bool {
        match chars.peek() {
            Some((_, c)) if *c == expected => {
                chars.next();
                true
            }
            _ => false,
        }
    }
}
