//! Provides a parselet for if statements.


use crate::parser::{
    Tokenizer,
    Token,
    TokenType,
    Expression,
    Parser,
    PrefixParselet,
};


pub struct IfParselet;

impl PrefixParselet for IfParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, _token: Token) -> Expression {
        let condition = match parser.parse(tokenizer) {
            Some(c) => c,
            None => todo!(),
        };

        let mut body: Vec<Expression> = Vec::new();

        while {
            let next = match tokenizer.peek() {
                Some(n) => n,
                None => todo!(),
            };

            next.get_type() != TokenType::EndIf
        } {
            let expr = parser.parse(tokenizer);
            if let Some(e) = expr {
                body.push(e);
            } else {
                todo!();
            }
        }

        tokenizer.next();

        Expression::If {
            condition: Box::new(condition),
            body,
        }
    }
}