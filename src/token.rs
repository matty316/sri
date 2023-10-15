#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TokenType {
    Ident,
    Int,
    Float,
    Plus,
    Minus,
    Star,
    Slash,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Eq,
    BangEq,
    Let,
    Fun,
    If,
    Else,
    ElseIf,
    LParen,
    RParen,
    Colon,
    Semicolon,
    String,
    Illegal,
    Eof
}

pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) literal: String
}