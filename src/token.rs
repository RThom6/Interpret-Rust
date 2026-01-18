pub struct Token {
    pub token_type: TokenType,
    pub lexeme: &'static str,
    pub line: usize,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenType {
    Plus,
    Minus,
    Divide,
    Multiply,
    BraceLeft,
    BraceRight,
    BracketLeft,
    BracketRight,
    ParenthesesLeft,
    ParenthesesRight,
    SemiColon,
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
    Identifier(String),
    Keyword(KeywordType),
    EOF,
} // TODO: Do I want invalid tokens? Optionally I could just error when I reach an invalid token? It would let me make a full trace of what's wrong with the input if I didn't error out immediately.

#[derive(Debug)]
pub enum KeywordType {
    If,
    Else,
    While,
    For,
    Return,
    Break,
    Continue,
    Null,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}

impl KeywordType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "if" => Some(Self::If),
            "else" => Some(Self::Else),
            "while" => Some(Self::While),
            "for" => Some(Self::For),
            "return" => Some(Self::Return),
            "break" => Some(Self::Break),
            "continue" => Some(Self::Continue),
            "null" => Some(Self::Null),
            _ => None,
        }
    }
}
