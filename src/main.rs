extern crate my_lang;

use my_lang::tokenizer::Tokenizer;
use my_lang::parser::Parser;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let mut file = match File::open(&args[1]) {
            Ok(file) => file,
            Err(err) => {
                println!("err: {:?}", err);
                return;
            }
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => {}
            Err(err) => {
                println!("err: {:?}", err);
                return;
            }
        };

        let mut tokenizer = Tokenizer::new(contents.chars().collect());
        let mut tokens = match tokenizer.tokenize() {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };

        println!("{:#?}", tokens);

        let mut parser = Parser::new(tokens);
        let mut ast = match parser.parse() {
            Ok(ast) => ast,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };

        println!("{:#?}", ast);

    } else {
        println!("usage: {} <filepath>", args[0]);
        return;
    }
}
