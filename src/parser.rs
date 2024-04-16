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
            TokenType::Function => self.parse_function_declaration(),
            _ => {
                self.errors.push(format!(
                    "Unhandled statement type: {:?}",
                    self.current_token.token_type
                ));
                None
            }
        }
    }
    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.token_type == t
    }
    fn parse_expression(&mut self) -> Option<Expression> {
        let token_type = self.current_token.token_type.clone();
        match token_type {
            TokenType::Ident(name) => {
                self.next_token();
                Some(Expression::Identifier(name))
            },
            TokenType::Int(value) => {
                self.next_token();
                Some(Expression::IntegerLiteral(value))
            },
            TokenType::Bool(value) => {
                self.next_token();
                Some(Expression::Boolean(value))
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
    fn parse_function_declaration(&mut self) -> Option<Statement> {
        if !self.expect_peek(TokenType::Function) {
            return None;
        }
        println!("Parsing function declaration. Current token: {:?}", self.current_token);
        self.next_token();
        let function_name = if let TokenType::Ident(name) = &self.current_token.token_type {
            name.clone()
        } else {
            self.errors.push("Expected function name".to_string());
            return None;
        };

        self.next_token();
        if !self.expect_peek(TokenType::LParen) {
            return None;
        }

        let parameters = self.parse_function_parameters();
        
        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let body = self.parse_block_statement();
        Some(Statement::Function(function_name, parameters, body))
    }

    fn parse_function_parameters(&mut self) -> Vec<String> {
        let mut parameters = Vec::new();

        if self.peek_token_is(TokenType::RParen) {
            self.next_token();
            return parameters;
        }
        self.next_token();
        if let TokenType::Ident(param) = &self.current_token.token_type {
            parameters.push(param.clone());
        }
        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            if let TokenType::Ident(param) = &self.current_token.token_type {
                parameters.push(param.clone());
            } else {
                self.errors.push("Expected parameter name".to_string());
                break;
            }
        }
        if !self.expect_peek(TokenType::RParen) {
            self.errors.push("Expected ')' after parameters".to_string());
        }
        parameters
    }

    fn parse_block_statement(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        self.next_token();
        while !self.current_token_is(TokenType::RBrace) && !self.current_token_is(TokenType::EOF) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }
        statements
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
    #[test]
    fn test_function_declarations() {
        let input = r#"
        function add(x, y) {
            return x + y;
        }
        "#;
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        assert_eq!(parser.current_token.token_type, TokenType::Function, "First token is not 'Function'");
    }
    
    #[test]
    fn test_function_keyword() {
        let input = "function";
        let mut lexer = Lexer::new(input.to_string());
        let token = lexer.next_token();
        assert_eq!(token.token_type, TokenType::Function, "Failed to recognize 'function' as a keyword");
    }
    #[test]
    fn test_improper_syntax() {
        let inputs = vec![
            "function { return; }",
            "function test(x, y { return x + y; }",
            "function test(x, y) return x + y;",
        ];
    
        for input in inputs {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            parser.parse_statement();
            assert!(!parser.errors.is_empty(), "Expected errors for input: {}", input);
        }
    }
    #[test]
    fn test_unexpected_token() {
        let input = r#"
        let x = function(y, z) return y + z; // Using 'function' in an expression improperly
        "#;
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        parser.parse_statement();
        assert!(!parser.errors.is_empty(), "Expected errors for misuse of 'function' keyword");
    }
       
    
}
