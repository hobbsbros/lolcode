//! Main executable for the Lolcode compiler.


pub mod tokenizer;


use std::{
    fs::{
        self,
        read_to_string,
    },
    env,
};

use tokenizer::{
    Tokenizer,
};


fn main() {
    // Do not use `unwrap`
    let filename: String = env::args().nth(1).unwrap();

    let file: String = read_to_string(&filename).unwrap();

    let tokenizer = Tokenizer::new(file);

    dbg!(&tokenizer.collect());
}