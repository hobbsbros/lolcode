//! Provides error handling for the Lolcode compiler.

use std::process::exit;

use colored::*;


pub enum Error {
    NoFilenameProvided,
    CouldNotRead (String),
    CouldNotOpen (String),
    CouldNotWrite (String),
    UnresolvedImport (String),
    UndeclaredIdentifier (String),
    InvalidOperator,
    CouldNotParse (String),
    ExpectedIdentifier,
    ExpectedDeclaration (String),
    ExpectedBeginProgram,
    ExpectedEndProgram,
    UnexpectedEof,
}

use Error::*;


fn print(s: String) {
    println!("{}: {}", "Error".red().bold(), s);
}


pub fn throw(error: Error) -> ! {
    println!("{}\n\n", "The LOLCODE Compiler".truecolor(255, 125, 0).bold());

    match error {
        NoFilenameProvided => print("No filename provided.".to_string()),
        CouldNotRead (s) => print(format!("Could not read file: {}", s)),
        CouldNotOpen (s) => print(format!("Could not open file: {}", s)),
        CouldNotWrite (s) => print(format!("Could not write file: {}", s)),
        UnresolvedImport (s) => print(format!("Unresolved import: {}", s)),
        UndeclaredIdentifier (s) => print(format!("Undeclared identifier: {}", s)),
        InvalidOperator => print(format!("Invalid operator used.")),
        CouldNotParse (s) => print(format!("Could not parse near token: {}", s)),
        ExpectedIdentifier => print(format!("Expected identifier.")),
        ExpectedDeclaration (s) => print(format!("Expected declaration, found token: {}", s)),
        ExpectedBeginProgram => print(format!("Expected token 'HAI'")),
        ExpectedEndProgram => print(format!("Expected token 'KTHXBYE'")),
        UnexpectedEof => print(format!("Unexpected end of file.")),
    }

    println!("Compiler exiting.");

    exit(0);
}