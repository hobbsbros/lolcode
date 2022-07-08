//! Provides a parselet for binary operations.


use crate::parser::{
    Tokenizer,
    Token,
    TokenType,
    Expression,
    Parser,
    InfixParselet,
};

use crate::error::{
    Error::*,
    throw,
};


pub struct BinOpParselet;

impl InfixParselet for BinOpParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, left: Expression, token: Token) -> Expression {
        let right: Expression = match parser.parse(token.get_type().into(), tokenizer) {
            Some(v) => v,
            None => throw(CouldNotParse (token.get_value())),
        };

        match token.get_type() {
            TokenType::Plus
            | TokenType::Minus
            | TokenType::Multiply
            | TokenType::Divide
            | TokenType::Greater
            | TokenType::Lesser => {
                Expression::BinOp {
                    left: Box::new(left),
                    op: token.get_type(),
                    right: Box::new(right),
                }
            },
            _ => throw(CouldNotParse (token.get_value())),
        }
    }
}