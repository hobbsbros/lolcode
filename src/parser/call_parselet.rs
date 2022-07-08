//! Provides a parselet for function calls.


use crate::parser::{
    Tokenizer,
    Token,
    TokenType,
    Expression,
    Parser,
    PrefixParselet,
};


pub struct CallParselet;

impl PrefixParselet for CallParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        let t = match tokenizer.next() {
            Some(n) => n,
            None => todo!(),
        };

        let identifier = match t.get_type() {
            TokenType::Identifier => t.get_value(),
            _ => todo!(),
        };

        let mut body: Vec<Expression> = Vec::new();

        while {
            let next = match tokenizer.peek() {
                Some(n) => n,
                None => todo!(),
            };

            next.get_type() != TokenType::EndCall
        } {
            let expr = parser.parse(token.get_type().into(), tokenizer);
            if let Some(e) = expr {
                body.push(e);
            } else {
                todo!();
            }
        }

        if let Some(t) = tokenizer.next() {
            if t.get_type() == TokenType::EndCall {
                // No problem!
            } else {
                todo!()
            }
        } else {
            todo!()
        }

        Expression::Call {
            identifier,
            args: body,
        }
    }
}