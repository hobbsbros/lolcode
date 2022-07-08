//! Provides a parselet for loops.


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


pub struct LoopParselet;

impl PrefixParselet for LoopParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        let label = match tokenizer.next() {
            Some(l) => l,
            None => throw(UnexpectedEof),
        };

        match label.get_type() {
            TokenType::Identifier => (),
            _ => throw(ExpectedIdentifier),
        };

        let mut body: Vec<Expression> = Vec::new();

        while {
            let next = match tokenizer.peek() {
                Some(n) => n,
                None => throw(UnexpectedEof),
            };

            next.get_type() != TokenType::EndLoop
        } {
            let expr = parser.parse(token.get_type().into(), tokenizer);
            if let Some(e) = expr {
                body.push(e);
            } else {
                throw(CouldNotParse (token.get_value()));
            }
        }

        if let Some(t) = tokenizer.next() {
            if t.get_type() != TokenType::EndLoop {
                throw(CouldNotParse (token.get_value()));
            }
        } else {
            throw(UnexpectedEof);
        }

        if let Some(t) = tokenizer.next() {
            if t.get_type() != TokenType::Identifier || t.get_value() != label.get_value() {
                throw(CouldNotParse (token.get_value()));
            }
        } else {
            throw(UnexpectedEof);
        }

        Expression::Loop {
            body,
        }
    }
}