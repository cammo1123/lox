use std::fmt;

use crate::object::Object;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, SemiColon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.
    Identifier, String, Number,

    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    EOF
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub literal: Object,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: impl Into<String>, literal: Object, line: usize) -> Self {
        Self {
            token_type: token_type,
            lexeme: lexeme.into(),
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} '{}' {}", self.token_type, self.lexeme, self.literal)
    }
}

