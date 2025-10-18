mod token_scanner;

pub mod token_scanner;

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
    EqualEqual,
    NotEqual,
    Equal,
    PlusEqual,
    MinusEqual,
    EqualPlus,
    EqualMinus,
    Number(u32),
    StringLiteral(String),
    Invalid(String),
    VariableName(String),
    Keyword(String),
}

static KEYWORDS: [String; 10] = [""]; // Placeholder until I make keywords

fn get_tokens(input: String) -> Vec<Token> {
    let mut char_indices = input.char_indices();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some((pos, ch)) = char_indices.next() {
        let token = match ch {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '=' => match char_indices.next_if_eq(&(pos + 1, '=')) {
                Some(_equals) => Token::EqualEqual,
                None => Token::Equal,
            },
            '{' => Token::BraceLeft,
            '}' => Token::BraceRight,
            '(' => Token::BracketLeft,
            ')' => Token::BracketRight,
            '[' => Token::ParenthesesLeft,
            ']' => Token::ParenthesesRight,
            '!' => match char_indices.next() {
                // Add cases for braces and functions i guess
                '=' => Token::NotEqual,
                _ => Token::Invalid("!".to_string()),
            },
            '"' => {
                let mut last_matched: char = '\0';

                let s: String = char_indices
                    .by_ref() // borrow mutable copy
                    .take_while(|(_pos, c)| {
                        // Keep track of last matched to invalidated if it doesn't end with '"' and we reach EOF
                        last_matched = *c;
                        *c != '"';
                    }) // Take from input stream until reaching '"'
                    .map(|(_pos, c)| c) // Map the characters onto c
                    .collect(); // Basically collects entire strings from stream. TODO: doesn't think about escaped characters but I can sort this later

                match last_matched {
                    '"' => Token::StringLiteral(s),
                    _ => Token::Invalid("Unterminated literal".to_string()),
                }

                Token::StringLiteral(s);
            }
            'a'..='z' | 'A'..='Z' => {
                let s: String = char_indices
                    .by_ref()
                    .take_while(|(_pos, c)| *c != "")
                    .map(|(_pos, c)| c)
                    .collect();

                if KEYWORDS.contains(s) {
                    Token::Keyword(s);
                } else {
                    Token::Variable(s);
                }
            }
            _ => Token::Invalid(format!("{}", ch)),
        };
        tokens.push(token);
    }
}
