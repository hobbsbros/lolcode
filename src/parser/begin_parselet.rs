//! Provides a parselet for BeginPgrm.


use crate::parser::{
    Tokenizer,
    Token,
    TokenType,
    Expression,
    Parser,
    PrefixParselet,
};

use crate::error::{
    Error::*,
    throw,
};


pub struct BeginParselet;

impl PrefixParselet for BeginParselet {
    fn parse(&self, _parser: &Parser, _tokenizer: &mut Tokenizer, token: Token) -> Expression {
        match token.get_type() {
            TokenType::BeginPgrm => Expression::BeginPgrm,
            _ => throw(ExpectedBeginProgram),
        }
    }
}