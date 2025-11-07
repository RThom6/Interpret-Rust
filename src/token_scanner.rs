#[allow(dead_code)]
#[derive(Debug)]
pub enum Token {
    Plus,
    Minus,
    BraceLeft,
    BraceRight,
    BracketLeft,
    BracketRight,
    ParenthesesLeft,
    ParenthesesRight,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
    EqualEqual,
    NotEqual,
    Equal,
    PlusEqual,
    MinusEqual,
    EqualPlus,
    EqualMinus,
    Number(u32),
    StringLiteral(String),
    CharLiteral(char),
    Invalid(String),
    VariableName(String),
    Keyword(String),
} // TODO: Do I want invalid tokens? Optionally I could just error when I reach an invalid token? It would let me make a full trace of what's wrong with the input if I didn't error out immediately.

#[allow(dead_code)]
static KEYWORDS: [&str; 10] = [
    "if", "else", "while", "for", "return", "break", "continue", "null", "a", "b",
]; // TODO: Make more keywords - replace a, b placeholders

pub struct TokenScanner {
    input: String,
    tokens: Vec<Token>,
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

    pub fn next_token(&mut self) -> Option<&Token> {
        if self.pos >= self.tokens.len() {
            return None;
        }

        let token = self.tokens.get(self.pos).unwrap();
        self.pos += 1;
        Some(token)
    }
}

impl TokenScanner {
    fn populate_tokens(&mut self) {
        let mut chars = self.input.char_indices().peekable();

        while let Some((_pos, ch)) = chars.next() {
            let token = match ch {
                ' ' | '\n' | '\t' | '\r' => continue,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '=' => {
                    if let Some((_, '=')) = chars.peek() {
                        chars.next();
                        Token::EqualEqual
                    } else {
                        Token::Equal
                    }
                }
                '>' => match chars.peek() {
                    Some((_, '=')) => {
                        chars.next();
                        Token::GreaterEqual
                    }
                    _ => Token::GreaterThan,
                },
                '<' => match chars.peek() {
                    Some((_, '=')) => {
                        chars.next();
                        Token::LessEqual
                    }
                    _ => Token::LessThan,
                },
                '{' => Token::BraceLeft,
                '}' => Token::BraceRight,
                '(' => Token::BracketLeft,
                ')' => Token::BracketRight,
                '[' => Token::ParenthesesLeft,
                ']' => Token::ParenthesesRight,
                '!' => {
                    if let Some((_, '=')) = chars.peek() {
                        chars.next();
                        Token::EqualEqual
                    } else {
                        Token::Invalid("!".to_string())
                    }
                }
                '"' => {
                    let mut s = String::new();

                    // Keep reading until another '"' or eof
                    // TODO: Handle escape sequences and invalid characters - Is this needed? Could be handled within things that work with strings.
                    while let Some((_, c)) = chars.next() {
                        if c == '"' {
                            break;
                        }

                        s.push(c);
                    }

                    // If end of string
                    if !s.ends_with('"') && chars.peek().is_none() {
                        Token::Invalid("Unterminated string literal".to_string())
                    } else {
                        Token::StringLiteral(s)
                    }
                }
                '\'' => {
                    let character: char = chars.next().unwrap().1;
                    if chars.next().unwrap().1 == '\'' {
                        Token::CharLiteral(character)
                    } else {
                        Token::Invalid("Unterminated character literal".to_string())
                    }
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut s = ch.to_string();

                    while let Some((_, c)) = chars.peek() {
                        if c.is_alphanumeric() {
                            s.push(*c);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    if KEYWORDS.contains(&s.as_str()) {
                        Token::Keyword(s)
                    } else {
                        Token::VariableName(s)
                    }
                }
                _ => Token::Invalid(format!("{}", ch)),
            };
            self.tokens.push(token);
        }
    }
}
