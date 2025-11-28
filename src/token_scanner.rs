use std::process::exit;

use crate::error::{Error, report};
use crate::token::{Keyword, Token, TokenType};

#[allow(dead_code)]
static KEYWORDS: [&str; 8] = [
    "if", "else", "while", "for", "return", "break", "continue", "null",
];

pub struct TokenScanner {
    input: String,
    tokens: Vec<TokenType>,
    pos: usize,
}

impl TokenScanner {
    pub fn new(input: String) -> Self {
        let mut scanner = TokenScanner {
            input,
            tokens: Vec::new(),
            pos: 0,
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

        while let Some((_, ch)) = chars.peek().cloned() {
            match ch {
                ' ' | '\n' | '\t' | '\r' => {
                    chars.next();
                }
                '+' => {
                    chars.next();
                    self.tokens.push(TokenType::Plus);
                }
                '-' => {
                    chars.next();
                    self.tokens.push(TokenType::Minus);
                }
                '=' => {
                    chars.next();
                    if Self::match_expected(&mut chars, '=') {
                        self.tokens.push(TokenType::EqualEqual);
                    } else {
                        self.tokens.push(TokenType::Equal);
                    }
                }
                '>' => {
                    chars.next();
                    if Self::match_expected(&mut chars, '=') {
                        self.tokens.push(TokenType::GreaterEqual);
                    } else {
                        self.tokens.push(TokenType::GreaterThan);
                    }
                }
                '<' => {
                    chars.next();
                    if Self::match_expected(&mut chars, '=') {
                        self.tokens.push(TokenType::LessEqual);
                    } else {
                        self.tokens.push(TokenType::LessThan);
                    }
                }
                '!' => {
                    chars.next();
                    if Self::match_expected(&mut chars, '=') {
                        self.tokens.push(TokenType::NotEqual);
                    } else {
                        self.tokens
                            .push(TokenType::Invalid("Unexpected '!'".into()));
                    }
                }
                '{' => {
                    chars.next();
                    self.tokens.push(TokenType::BraceLeft);
                }
                '}' => {
                    chars.next();
                    self.tokens.push(TokenType::BraceRight);
                }
                '(' => {
                    chars.next();
                    self.tokens.push(TokenType::BracketLeft);
                }
                ')' => {
                    chars.next();
                    self.tokens.push(TokenType::BracketRight);
                }
                '[' => {
                    chars.next();
                    self.tokens.push(TokenType::ParenthesesLeft);
                }
                ']' => {
                    chars.next();
                    self.tokens.push(TokenType::ParenthesesRight);
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
                        self.tokens
                            .push(TokenType::Invalid("Unterminated string literal".into()));
                    } else {
                        self.tokens.push(TokenType::StringLiteral(s));
                    }
                }
                '\'' => {
                    chars.next();

                    let char_value = match chars.next() {
                        Some((_, c)) => c,
                        None => {
                            self.tokens
                                .push(TokenType::Invalid("Unterminated character literal".into()));
                            continue;
                        }
                    };

                    match chars.next() {
                        Some((_, '\'')) => {
                            self.tokens.push(TokenType::CharLiteral(char_value));
                        }
                        _ => {
                            self.tokens
                                .push(TokenType::Invalid("Unterminated character literal".into()));
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
                        self.tokens.push(TokenType::Keyword(s));
                    } else {
                        self.tokens.push(TokenType::Identifier(s));
                    }
                }
                _ => {
                    report(Error::new("Invalid Character Placeholder", 16));
                    exit(1);
                }
            }
        }
    }

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
