#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Null,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    String,
    Number,
    Identifier
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    value: Option<String>,
    line: usize
}

impl Token {
    pub fn new(token_type: TokenType, value: Option<String>, line: usize) -> Self {
        Self {
            token_type,
            value,
            line
        }
    }
}

