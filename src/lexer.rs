// src/lexer.rs

use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let token;
        self.skip_whitespace();

        match self.ch {
            '+' => {
                token = Token {
                    token_type: TokenType::Plus,
                    literal: self.ch.to_string(),
                };
            },
            '0'..='9' => {
                let literal = self.read_number();
                token = Token {
                    token_type: TokenType::Int(literal.parse::<i64>().unwrap()),
                    literal,
                };
                return token;
            },
            _ if Lexer::is_letter(self.ch) => {
                let literal = self.read_identifier();
                token = Token {
                    token_type: TokenType::Ident(literal.clone()),
                    literal,
                };
                return token;
            },
            '\0' => {
                token = Token {
                    token_type: TokenType::EOF,
                    literal: "".to_string(),
                };
            },
            _ => {
                token = Token {
                    token_type: TokenType::EOF,
                    literal: "".to_string(),
                };
            },
        }
        self.read_char();
        token
    }

    fn read_number(&mut self) -> String {
        let start_position = self.position;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        self.input[start_position..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let start_position = self.position;
        while Lexer::is_letter(self.ch) {
            self.read_char();
        }
        self.input[start_position..self.position].to_string()
    }

    fn is_letter(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }
}
