use std::iter::Peekable;
use std::process::exit;
use std::str::CharIndices;
use std::thread::sleep;
use std::time::Duration;

use crate::error::{Error, report};
use crate::token::{Keyword, Token, TokenType};

#[allow(dead_code)]
static KEYWORDS: [&str; 8] = [
    "if", "else", "while", "for", "return", "break", "continue", "null",
];

pub struct TokenScanner {
    input: String,
    chars: Peekable<CharIndices<'static>>,
    tokens: Vec<TokenType>,
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
            chars: "".char_indices().peekable(),
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
        while let Some((a, ch)) = chars.peek().cloned() {
            match ch {
                ' ' | '\t' | '\r' => {
                    chars.next();
                }
                '\n' => {
                    self.line += 1;
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
                        self.tokens.push(TokenType::Keyword(Keyword::from_str(s)));
                    } else {
                        self.tokens.push(TokenType::Identifier(s));
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
                        self.tokens.push(TokenType::Divide);
                    }
                }
                '*' => {
                    self.tokens.push(TokenType::Multiply);
                }
                '0'..='9' => {
                    let mut s = "".to_owned();
                    s.push(ch);

                    while let Some((_, c)) = chars.peek() {
                        if !c.is_digit(10) {
                            self.tokens
                                .push(TokenType::Number(s.parse::<u32>().unwrap()));
                            break;
                        }

                        s.push(ch);
                        chars.next();
                    }
                }
                ';' => self.tokens.push(TokenType::SemiColon),
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
