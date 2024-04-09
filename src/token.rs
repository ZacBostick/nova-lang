// src/token.rs

#[derive(Debug, PartialEq)]
pub enum TokenType {
    EOF,
    Ident(String),
    Int(i64),
    Plus,
    
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
