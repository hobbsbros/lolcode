//! Main executable for the Lolcode compiler.


pub mod tokenizer;
pub mod parser;
pub mod emitter;


use std::{
    io::Write,
    fs::{
        OpenOptions,
        read_to_string,
    },
    env,
};

use colored::*;

use tokenizer::Tokenizer;
use parser::Parser;
use emitter::Emitter;


fn main() {
    println!("{}", "The LOLCODE Compiler".truecolor(255, 125, 0).bold());

    // TODO: do not use `unwrap`
    let filename: String = env::args().nth(1).unwrap();

    let file: String = read_to_string(&filename).unwrap();

    let mut tokenizer = Tokenizer::new(file);

    let parser = Parser::new();

    let emitter = Emitter::new(&mut tokenizer, &parser);

    let output_filename = "out.c".to_string();

    // Open a file for output
    let mut output = match OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_filename.to_owned())
    {
        Ok(f) => f,
        Err(_) => todo!(),
    };

    match output.write_all(emitter.output().as_bytes()) {
        Ok(_) => (),
        Err(_) => todo!(),
    };
}