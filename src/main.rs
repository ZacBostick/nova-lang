use std::env;
use std::fs;

mod token;
mod lexer;

use crate::lexer::Lexer;
use crate::token::TokenType;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: nova_compiler <filename>");
        std::process::exit(1);
    }
    let filename = &args[1];

    let input = fs::read_to_string(filename).expect("Could not read file");

    let mut lexer = Lexer::new(input);

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token.token_type == TokenType::EOF {
            break;
        }
    }
}
