//! Provides a parselet for identifiers.


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


pub struct IdentifierParselet;

impl PrefixParselet for IdentifierParselet {
    fn parse(&self, _parser: &Parser, _tokenizer: &mut Tokenizer, token: Token) -> Expression {
        match token.get_type() {
            TokenType::Identifier => Expression::Identifier (token.get_value()),
            _ => throw(ExpectedIdentifier),
        }
    }
}