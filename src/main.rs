mod parser;
mod lexer;
mod ast;

use std::fs::read_to_string;
use crate::parser::parser::Parser;

fn main() {
    let src = read_to_string("./examples/script.sc").unwrap();
    let ast = Parser::new(src).parse();

    println!("{ast:#?}");
}
