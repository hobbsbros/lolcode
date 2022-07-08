//! Provides a parselet for include statements.


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


pub struct IncludeParselet;

impl PrefixParselet for IncludeParselet {
    fn parse(&self, _parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        let next = match tokenizer.next() {
            Some(n) => n,
            None => throw(UnexpectedEof),
        };

        match next.get_type() {
            TokenType::Identifier => (),
            _ => throw(ExpectedIdentifier),
        };
        
        match token.get_type() {
            TokenType::Include => Expression::Include {
                module: next.get_value()
            },
            _ => throw(CouldNotParse (token.get_value())),
        }
    }
}