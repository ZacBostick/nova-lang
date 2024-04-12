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
    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap_or('\0')
        }
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
    fn skip_comment(&mut self) {
        while self.ch != '\n' && self.ch != '\0' {
            self.read_char();
        }
    }
    fn read_string(&mut self) -> String {
        let start_position = self.position + 1; 
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == '\0' { 
                break;
            }
        }
        let end_position = self.position;
        if self.ch == '"' {
            self.read_char();
        }
        self.input[start_position..end_position].to_string()
    }
    fn skip_multi_line_comment(&mut self) {
        loop {
            self.read_char();
            if self.ch == '*' && self.peek_char() == '/' {
                self.read_char(); 
                self.read_char(); 
                break;
            } else if self.ch == '\0' {
                break;
            }
        }
    }
    
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.ch == '/' {
            if self.peek_char() == '/' {
                self.skip_comment();
                self.read_char();
                return self.next_token();
            } else if self.peek_char() == '*' {
                self.skip_multi_line_comment();
                return self.next_token();
            }
        }
        
        let token = match self.ch {
            '+' => Token { token_type: TokenType::Plus, literal: self.ch.to_string() },
            '-' => Token { token_type: TokenType::Minus, literal: self.ch.to_string() },
            '*' => Token { token_type: TokenType::Asterisk, literal: self.ch.to_string() },
            '/' => Token { token_type: TokenType::Slash, literal: self.ch.to_string() },
            '=' => Token { token_type: TokenType::Equal, literal: self.ch.to_string() },
            '!' => Token { token_type: TokenType::NotEqual, literal: self.ch.to_string() },
            '<' => Token { token_type: TokenType::LessThan, literal: self.ch.to_string() },
            '>' => Token { token_type: TokenType::GreaterThan, literal: self.ch.to_string() },
            '(' => Token { token_type: TokenType::LParen, literal: self.ch.to_string() },
            ')' => Token { token_type: TokenType::RParen, literal: self.ch.to_string() },
            '{' => Token { token_type: TokenType::LBrace, literal: self.ch.to_string() },
            '}' => Token { token_type: TokenType::RBrace, literal: self.ch.to_string() },
            ',' => Token { token_type: TokenType::Comma, literal: self.ch.to_string() },
            ';' => Token { token_type: TokenType::Semicolon, literal: self.ch.to_string() },
            '"' => {
                let literal = self.read_string();
                return Token { token_type: TokenType::Str(literal.clone()), literal };
            },            
            
            '0'..='9' => {
            let literal = self.read_number();
                return Token { 
                    token_type: TokenType::Int(literal.parse::<i64>().expect("Failed to parse integer")),
                    literal 
                };
            },
            _ if Lexer::is_letter(self.ch) => {
                let literal = self.read_identifier();
                let token_type = match literal.as_str() {
                    "let" => TokenType::Let,
                    "fn" => TokenType::Fn,
                    "if" => TokenType::If,
                    "else" => TokenType::Else,
                    "return" => TokenType::Return,
                    "true" => TokenType::True,
                    "false" => TokenType::False,
                    _ => TokenType::Ident(literal.clone()),
                };
                return Token { token_type, literal };
            },
            '\0' => Token { 
                token_type: TokenType::EOF, 
                literal: "".to_string() 
            },
            _ => Token { token_type: TokenType::Illegal, literal: self.ch.to_string() },
        };
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
#[cfg(test)]
mod tests {
    use super::*;
    fn assert_token_type(token: &Token, expected_type: &TokenType) {
        match (expected_type, &token.token_type) {
            (TokenType::Int(_), TokenType::Int(_)) => (),
            (TokenType::Str(_), TokenType::Str(_)) => (),
            (expected, actual) => assert_eq!(expected, actual),
        }
    }
    #[test]
    fn test_operators() {
        let input = "+ - * / = ! < >";
        let mut lexer = Lexer::new(input.to_string());
        let expected_tokens = vec![
            TokenType::Plus, TokenType::Minus, TokenType::Asterisk, TokenType::Slash,
            TokenType::Equal, TokenType::NotEqual, TokenType::LessThan, TokenType::GreaterThan,
        ];

        for expected in expected_tokens {
            let token = lexer.next_token();
            assert_token_type(&token, &expected);
        }
    }
    #[test]
    fn test_numbers() {
        let input = "123 456 789";
        let mut lexer = Lexer::new(input.to_string());
        let expected_values = [123, 456, 789];
    
        for &expected in expected_values.iter() {
            let token = lexer.next_token();
            if let TokenType::Int(value) = token.token_type {
                assert_eq!(value, expected);
            } else {
                panic!("Expected integer token, found {:?}", token.token_type);
            }
        }
    }
    #[test]
    fn test_strings() {
        let input = "\"hello\" \"world\"";
        let mut lexer = Lexer::new(input.to_string());

        for expected in ["hello", "world"].iter() {
            let token = lexer.next_token();
            match token.token_type {
                TokenType::Str(ref s) if s == expected => (),
                _ => panic!("Expected Str({}), found {:?}", expected, token.token_type),
            }
        }
    }
    #[test]
    fn test_whitespace_and_comments() {
        let input = r#"
            // This is a single-line comment
            /* This is a
               multi-line comment */
            42
            // Another comment
        "#;
        let mut lexer = Lexer::new(input.to_string());
        let token = lexer.next_token();
        match token.token_type {
            TokenType::Int(value) => assert_eq!(value, 42),
            _ => panic!("Expected Int, found {:?}", token.token_type),
        }
        assert_eq!(lexer.next_token().token_type, TokenType::EOF);
    }
    #[test]
    fn test_keywords_and_identifiers() {
        let input = "let variable = if else fn return";
        let mut lexer = Lexer::new(input.to_string());
        let expected_tokens = [
            TokenType::Let, TokenType::Ident("variable".to_string()), TokenType::Equal,
            TokenType::If, TokenType::Else, TokenType::Fn, TokenType::Return,
        ];
        for expected in expected_tokens.iter() {
            let token = lexer.next_token();
            assert_token_type(&token, expected);
        }
    }
    #[test]
    fn test_combined_syntax() {
        let input = "fn add(x, y) { x + y; }";
        let mut lexer = Lexer::new(input.to_string());
        let expected_tokens = [
            TokenType::Fn, TokenType::Ident("add".to_string()),
            TokenType::LParen, TokenType::Ident("x".to_string()), TokenType::Comma, TokenType::Ident("y".to_string()),
            TokenType::RParen, TokenType::LBrace, TokenType::Ident("x".to_string()), TokenType::Plus,
            TokenType::Ident("y".to_string()), TokenType::Semicolon, TokenType::RBrace
        ];
        for expected in expected_tokens.iter() {
            let token = lexer.next_token();
            assert_token_type(&token, expected);
        }
    }
    #[test]
    fn test_illegal_characters() {
        let input = "@ # $ % ^ &";
        let mut lexer = Lexer::new(input.to_string());
        for _ in 0..6 {
            let token = lexer.next_token();
            assert_eq!(token.token_type, TokenType::Illegal);
        }
    }
    #[test]
    fn test_eof() {
        let input = "variable";
        let mut lexer = Lexer::new(input.to_string());
        assert_token_type(&lexer.next_token(), &TokenType::Ident("variable".to_string()));
        assert_eq!(lexer.next_token().token_type, TokenType::EOF);
        assert_eq!(lexer.next_token().token_type, TokenType::EOF);
    }
                       
}
