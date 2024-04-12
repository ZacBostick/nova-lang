// src/ast.rs

#[derive(PartialEq, Debug)]
pub enum Statement {
    Let(String, Expression),
    Return(Expression),
    Expression(Expression),
    If(Box<Expression>, Box<Vec<Statement>>, Option<Box<Vec<Statement>>>),
    Function(String, Vec<String>, Vec<Statement>),
}

#[derive(PartialEq, Debug)]
pub enum Expression {
    Identifier(String),
    IntegerLiteral(i64),
    Prefix(String, Box<Expression>),
    Infix(String, Box<Expression>, Box<Expression>),
    Boolean(bool),
    If(Box<Expression>, Box<Statement>, Option<Box<Statement>>),
    Function(Vec<String>, Vec<Statement>),
    Call(Box<Expression>, Vec<Expression>),
}
