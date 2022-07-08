//! Provides a parselet for loops.


use crate::parser::{
    Tokenizer,
    Token,
    TokenType,
    Expression,
    Parser,
    PrefixParselet,
};


pub struct LoopParselet;

impl PrefixParselet for LoopParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        let label = match tokenizer.next() {
            Some(l) => l,
            None => todo!(),
        };

        match label.get_type() {
            TokenType::Identifier => (),
            _ => todo!(),
        };

        let mut body: Vec<Expression> = Vec::new();

        while {
            let next = match tokenizer.peek() {
                Some(n) => n,
                None => todo!(),
            };

            next.get_type() != TokenType::EndLoop
        } {
            let expr = parser.parse(token.get_type().into(), tokenizer);
            if let Some(e) = expr {
                body.push(e);
            } else {
                todo!();
            }
        }

        if let Some(t) = tokenizer.next() {
            if t.get_type() == TokenType::EndLoop {
                // No problem!
            } else {
                todo!()
            }
        } else {
            todo!()
        }

        if let Some(t) = tokenizer.next() {
            if t.get_type() == TokenType::Identifier && t.get_value() == label.get_value() {
                // No problem!
            } else {
                todo!()
            }
        } else {
            todo!()
        }

        Expression::Loop {
            body,
        }
    }
}