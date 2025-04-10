mod ast;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("digga");
    let input = input.trim(); 
    let mut lexer = ast::lexer::Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }
    println!("{:?}", tokens);

}
