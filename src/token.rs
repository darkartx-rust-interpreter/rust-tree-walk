use std::fmt;

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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenType::*;

        match self.token_type {
            LeftParen => write!(f, "("),
            RightParen => write!(f, ")"),
            LeftBrace => write!(f, "{{"),
            RightBrace => write!(f, "}}"),
            Comma => write!(f, ","),
            Dot => write!(f, "."),
            Minus => write!(f, "-"),
            Plus => write!(f, "+"),
            Semicolon => write!(f, ";"),
            Slash => write!(f, "/"),
            Star => write!(f, "*"),
            Bang => write!(f, "!"),
            BangEqual => write!(f, "!="),
            Equal => write!(f, "="),
            EqualEqual => write!(f, "=="),
            Greater => write!(f, ">"),
            GreaterEqual => write!(f, ">="),
            Less => write!(f, "<"),
            LessEqual => write!(f, "<="),
            And => write!(f, "and"),
            Class => write!(f, "class"),
            Else => write!(f, "else"),
            False => write!(f, "false"),
            Fun => write!(f, "fun"),
            For => write!(f, "for"),
            If => write!(f, "if"),
            Null => write!(f, "null"),
            Or => write!(f, "or"),
            Print => write!(f, "print"),
            Return => write!(f, "return"),
            Super => write!(f, "super"),
            This => write!(f, "this"),
            True => write!(f, "true"),
            Var => write!(f, "var"),
            While => write!(f, "while"),
            String => {
                let value = self.value.as_ref().unwrap();
                write!(f, "\"{}\"", value)
            },
            Number => {
                let number = self.value.as_ref().unwrap();
                write!(f, "{}", number)
            },
            Identifier => {
                let iden = self.value.as_ref().unwrap();
                write!(f, "{}", iden)
            }
        }
    }
}

