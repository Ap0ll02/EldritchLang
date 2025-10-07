use logos::Logos;
mod lexer;
use lexer::Token;

fn main() {
    
    for token in Token::lexer("Hello 69.0") {
        match token {
            Ok(tok) => {
                println!("{:?}", tok);
            }
            Err(e) => {
                println!("{:?}", e);
                return;
            }
        }
    }
    println!("Hello, world!");
}
