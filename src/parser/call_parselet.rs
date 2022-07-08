//! Provides a parselet for function calls.


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


pub struct CallParselet;

impl PrefixParselet for CallParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        let t = match tokenizer.next() {
            Some(n) => n,
            None => throw(UnexpectedEof),
        };

        let identifier = match t.get_type() {
            TokenType::Identifier => t.get_value(),
            _ => throw(ExpectedIdentifier),
        };

        let mut body: Vec<Expression> = Vec::new();

        while {
            let next = match tokenizer.peek() {
                Some(n) => n,
                None => throw(UnexpectedEof),
            };

            next.get_type() != TokenType::EndCall
        } {
            let expr = parser.parse(token.get_type().into(), tokenizer);
            if let Some(e) = expr {
                body.push(e);
            } else {
                throw(CouldNotParse (token.get_value()));
            }
        }

        if let Some(t) = tokenizer.next() {
            if t.get_type() == TokenType::EndCall {
                // No problem!
            } else {
                throw(CouldNotParse (t.get_value()));
            }
        } else {
            throw(UnexpectedEof);
        }

        Expression::Call {
            identifier,
            args: body,
        }
    }
}