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
    Identifier,
    Query,
    Colon
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenType::*;

        match self {
            LeftParen => write!(f, "LeftParen"),
            RightParen => write!(f, "RightParen"),
            LeftBrace => write!(f, "LeftBrace"),
            RightBrace => write!(f, "RightBrace"),
            Comma => write!(f, "Comma"),
            Dot => write!(f, "Dot"),
            Minus => write!(f, "Minus"),
            Plus => write!(f, "Plus"),
            Semicolon => write!(f, "Semicolon"),
            Slash => write!(f, "Slash"),
            Star => write!(f, "Star"),
            Bang => write!(f, "Bang"),
            BangEqual => write!(f, "BangEqual"),
            Equal => write!(f, "Equal"),
            EqualEqual => write!(f, "EqualEqual"),
            Greater => write!(f, "Greater"),
            GreaterEqual => write!(f, "GreaterEqual"),
            Less => write!(f, "Less"),
            LessEqual => write!(f, "LessEqual"),
            And => write!(f, "And"),
            Class => write!(f, "Class"),
            Else => write!(f, "Else"),
            False => write!(f, "False"),
            Fun => write!(f, "Fun"),
            For => write!(f, "For"),
            If => write!(f, "If"),
            Null => write!(f, "Null"),
            Or => write!(f, "Or"),
            Print => write!(f, "Print"),
            Return => write!(f, "Return"),
            Super => write!(f, "Super"),
            This => write!(f, "This"),
            True => write!(f, "True"),
            Var => write!(f, "Var"),
            While => write!(f, "While"),
            String => write!(f, "String"),
            Number => write!(f, "Number"),
            Identifier => write!(f, "Identifier"),
            Query => write!(f, "Query"),
            Colon => write!(f, "Colon"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: Option<String>,
    line: usize
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: Option<String>, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line
        }
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn lexeme(&self) -> Option<&str> {
        self.lexeme.as_ref().map(String::as_str)
    }

    pub fn line(&self) -> usize {
        self.line
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
            Query => write!(f, "?"),
            Colon => write!(f, ":"),
            String => {
                let value = self.lexeme().unwrap();
                write!(f, "\"{}\"", value)
            },
            Number => {
                let number = self.lexeme().unwrap();
                write!(f, "{}", number)
            },
            Identifier => {
                let iden = self.lexeme().unwrap();
                write!(f, "{}", iden)
            }
        }
    }
}

