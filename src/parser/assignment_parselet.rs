//! Provides a parselet for assignments.


use crate::parser::{
    Tokenizer,
    Token,
    Expression,
    Parser,
    InfixParselet,
};

use crate::error::{
    Error::*,
    throw,
};


pub struct AssignmentParselet;

impl InfixParselet for AssignmentParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, left: Expression, token: Token) -> Expression {
        let value: Expression = match parser.parse(token.get_type().into(), tokenizer) {
            Some(v) => v,
            None => throw(CouldNotParse (token.get_value())),
        };

        if let Expression::Identifier (s) = left {
            Expression::Assignment {
                identifier: s,
                value: Box::new(value),
            }
        } else {
            throw(ExpectedIdentifier);
        }
    }
}