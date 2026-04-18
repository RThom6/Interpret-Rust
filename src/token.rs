#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Plus,
    Minus,
    Divide,
    Multiply,
    Bang,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Plus,
    Minus,
    Divide,
    Multiply,
    Bang,
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
    Number,
    StringLiteral,
    CharLiteral,
    Invalid,
    Identifier,
    Keyword,
    EOF,
}

impl TokenType {
    pub fn kind(&self) -> TokenKind {
        match self {
            TokenType::Plus => TokenKind::Plus,
            TokenType::Minus => TokenKind::Minus,
            TokenType::Divide => TokenKind::Divide,
            TokenType::Multiply => TokenKind::Multiply,
            TokenType::Bang => TokenKind::Bang,
            TokenType::BraceLeft => TokenKind::BraceLeft,
            TokenType::BraceRight => TokenKind::BraceRight,
            TokenType::BracketLeft => TokenKind::BracketLeft,
            TokenType::BracketRight => TokenKind::BracketRight,
            TokenType::ParenthesesLeft => TokenKind::ParenthesesLeft,
            TokenType::ParenthesesRight => TokenKind::ParenthesesRight,
            TokenType::SemiColon => TokenKind::SemiColon,
            TokenType::GreaterThan => TokenKind::GreaterThan,
            TokenType::LessThan => TokenKind::LessThan,
            TokenType::GreaterEqual => TokenKind::GreaterEqual,
            TokenType::LessEqual => TokenKind::LessEqual,
            TokenType::EqualEqual => TokenKind::EqualEqual,
            TokenType::NotEqual => TokenKind::NotEqual,
            TokenType::Equal => TokenKind::Equal,
            TokenType::PlusEqual => TokenKind::PlusEqual,
            TokenType::MinusEqual => TokenKind::MinusEqual,
            TokenType::EqualPlus => TokenKind::EqualPlus,
            TokenType::EqualMinus => TokenKind::EqualMinus,
            TokenType::Number(_) => TokenKind::Number,
            TokenType::StringLiteral(_) => TokenKind::StringLiteral,
            TokenType::CharLiteral(_) => TokenKind::CharLiteral,
            TokenType::Invalid(_) => TokenKind::Invalid,
            TokenType::Identifier(_) => TokenKind::Identifier,
            TokenType::Keyword(_) => TokenKind::Keyword,
            TokenType::EOF => TokenKind::EOF,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
            lexeme: lexeme.to_string(),
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
