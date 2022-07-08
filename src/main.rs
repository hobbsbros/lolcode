//! Main executable for the Lolcode compiler.


pub mod tokenizer;
pub mod parser;


use std::{
    fs::{
        self,
        read_to_string,
    },
    env,
};

use tokenizer::Tokenizer;
use parser::Parser;


fn main() {
    // Do not use `unwrap`
    let filename: String = env::args().nth(1).unwrap();

    let file: String = read_to_string(&filename).unwrap();

    let mut tokenizer = Tokenizer::new(file);

    let parser = Parser::new();

    dbg!(&parser.parse_all(&mut tokenizer));
}