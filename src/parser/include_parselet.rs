//! Provides a parselet for include statements.


use crate::parser::{
    Tokenizer,
    Token,
    TokenType,
    Expression,
    Parser,
    PrefixParselet,
};


pub struct IncludeParselet;

impl PrefixParselet for IncludeParselet {
    fn parse(&self, _parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        let next = match tokenizer.next() {
            Some(n) => n,
            None => todo!(),
        };

        match next.get_type() {
            TokenType::Identifier => (),
            _ => todo!(),
        };
        
        match token.get_type() {
            TokenType::Include => Expression::Include {
                module: next.get_value()
            },
            _ => todo!(),
        }
    }
}