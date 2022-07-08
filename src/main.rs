//! Main executable for the Lolcode compiler.


pub mod tokenizer;
pub mod parser;
pub mod emitter;
pub mod error;


use std::{
    io::Write,
    fs::{
        OpenOptions,
        read_to_string,
    },
    env,
};

use tokenizer::Tokenizer;
use parser::Parser;
use emitter::Emitter;
use error::{
    Error::*,
    throw,
};


fn main() {
    let filename: String = match env::args().nth(1) {
        Some(f) => f,
        None => throw(NoFilenameProvided),
    };

    let file: String = match read_to_string(&filename) {
        Ok(f) => f,
        Err(_) => throw(CouldNotRead (filename)),
    };

    // Tokenize, parse, and emit code
    let mut tokenizer = Tokenizer::new(file);
    let parser = Parser::new();
    let emitter = Emitter::new(&mut tokenizer, &parser);

    // Construct a filename for the output
    let mut output_filename = filename.clone();
    output_filename.truncate(output_filename.len() - 4);
    output_filename.push_str(".c");

    // Open a file for output
    let mut output = match OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_filename.to_owned())
    {
        Ok(f) => f,
        Err(_) => throw(CouldNotOpen (output_filename)),
    };

    match output.write_all(emitter.output().as_bytes()) {
        Ok(_) => (),
        Err(_) => throw(CouldNotWrite (output_filename)),
    };
}