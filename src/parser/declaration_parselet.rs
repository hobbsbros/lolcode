//! Provides a parselet for declarations.


use crate::parser::{
    Tokenizer,
    Token,
    TokenType,
    Expression,
    Parser,
    PrefixParselet,
};


pub struct DeclarationParselet;

impl PrefixParselet for DeclarationParselet {
    fn parse(&self, _parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        match token.get_type() {
            TokenType::Declaration => {
                let identifier = match tokenizer.peek() {
                    Some(i) => i,
                    None => todo!(),
                };

                match identifier.get_type() {
                    TokenType::Identifier => {
                        tokenizer.next();
                        Expression::Declaration {
                            identifier: identifier.get_value()
                        }
                    },
                    _ => todo!(),
                }
            },
            _ => todo!(),
        }
    }
}