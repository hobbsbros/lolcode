//! Provides a parselet for literals.


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


pub struct LiteralParselet;

impl PrefixParselet for LiteralParselet {
    fn parse(&self, _parser: &Parser, _tokenizer: &mut Tokenizer, token: Token) -> Expression {
        match token.get_type() {
            // Note: it's ok to use `unwrap` here because we already checked in the tokenizer
            // that this will parse to an `i32`
            TokenType::Int => Expression::Int (str::parse::<i32>(&token.get_value()).unwrap()),
            _ => throw(CouldNotParse (token.get_value())),
        }
    }
}