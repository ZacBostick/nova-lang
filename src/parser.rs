// src/parser.rs

use crate::token::{Token, TokenType};
use crate::lexer::Lexer;
use crate::ast::{Expression, Statement};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
            peek_token,
            errors: Vec::new(),
        }
    }

    fn next_token(&mut self) {
        std::mem::swap(&mut self.current_token, &mut self.peek_token);
        self.peek_token = self.lexer.next_token();
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token.token_type == t {
            self.next_token();
            true
        } else {
            self.errors.push(format!(
                "Expected next token to be {:?}, got {:?} instead",
                t, self.peek_token.token_type
            ));
            false
        }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => {
                println!("Unhandled statement type: {:?}", self.current_token.token_type);
                None
            }
        }
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        match &self.current_token.token_type {
            TokenType::Ident(name) => {
                let identifier = name.clone();
                self.next_token();
                Some(Expression::Identifier(identifier))
            },
            TokenType::Int(value) => {
                let number = *value;
                self.next_token();
                Some(Expression::IntegerLiteral(number))
            },
            _ => {
                self.errors.push(format!("Unexpected token in expression: {:?}", self.current_token.token_type));
                None
            }
        }
    }
    
    fn parse_let_statement(&mut self) -> Option<Statement> {
        self.next_token();
        if let TokenType::Ident(name) = &self.current_token.token_type {
            let variable_name = name.clone();
            self.next_token();
            if self.current_token.token_type == TokenType::Equal {
                self.next_token();
                if let Some(expression) = self.parse_expression() {
                    if self.current_token_is(TokenType::Semicolon) {
                        self.next_token();
                        Some(Statement::Let(variable_name, expression))
                    } else {
                        self.errors.push("Expected semicolon at end of let statement".to_string());
                        None
                    }
                } else {
                    None
                }
            } else {
                self.errors.push("Expected '=' after variable name".to_string());
                None
            }
        } else {
            self.errors.push("Expected identifier after 'let'".to_string());
            None
        }
    }
    
    fn parse_expression_statement(&mut self) -> Option<Statement> {
        match &self.current_token.token_type {
            TokenType::Ident(name) => {
                let expression = Expression::Identifier(name.clone());
                self.next_token();
                if self.current_token_is(TokenType::Semicolon) {
                    self.next_token();
                    Some(Statement::Expression(expression))
                } else {
                    self.errors.push("Expected semicolon at end of expression statement".to_string());
                    None
                }
            },
            _ => {
                self.errors.push(format!("Unexpected token in expression statement: {:?}", self.current_token.token_type));
                None
            }
        }
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();
        let expr = if let Some(expr) = self.parse_expression() {
            expr
        } else {
            return None;
        };
    
        if !self.current_token_is(TokenType::Semicolon) {
            self.errors.push("Expected semicolon at the end of return statement".to_string());
            return None;
        }
        self.next_token();
        Some(Statement::Return(expr))
    }
    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    fn current_token_is(&self, t: TokenType) -> bool {
        self.current_token.token_type == t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_let_statements() {
        let input = "let x = 5;";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        if let Some(statement) = parser.parse_statement() {
            assert!(matches!(statement, Statement::Let(name, _) if name == "x"));
        } else {
            assert!(false, "Failed to parse 'let' statement");
        }
        assert!(parser.errors.is_empty(), "Parser errors: {:?}", parser.errors);
    }
    
    #[test]
    fn test_return_statements() {
        let input = "return 123;";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let statement = parser.parse_return_statement().unwrap();
    
        match statement {
            Statement::Return(Expression::IntegerLiteral(value)) => assert_eq!(value, 123),
            _ => panic!("Expected a Return statement with an IntegerLiteral, found {:?}", statement),
        }
    
        assert!(parser.errors.is_empty(), "There were errors while parsing: {:?}", parser.errors());
    }
    
    #[test]
    fn test_expression_statements() {
        let input = "example;";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        assert_eq!(parser.current_token.token_type, TokenType::Ident("example".to_string()));
        if let Some(statement) = parser.parse_expression_statement() {
            match statement {
                Statement::Expression(Expression::Identifier(name)) => assert_eq!(name, "example"),
                _ => panic!("Expected Expression statement, found {:?}", statement),
            }
        } else {
            panic!("Failed to parse expression statement, errors: {:?}", parser.errors);
        }
    }  
}
