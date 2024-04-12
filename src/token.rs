// src/token.rs

#[derive(Debug, PartialEq)]
pub enum TokenType {
    EOF,
    Ident(String),
    Int(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    Illegal,
    Let,
    If,
    Else,
    Fn,
    Return,
    True,
    False,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Str(String),
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}