//! Provides a parselet for declarations.


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


pub struct DeclarationParselet;

impl PrefixParselet for DeclarationParselet {
    fn parse(&self, _parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        match token.get_type() {
            TokenType::Declaration => {
                let identifier = match tokenizer.peek() {
                    Some(i) => i,
                    None => throw(UnexpectedEof),
                };

                match identifier.get_type() {
                    TokenType::Identifier => {
                        tokenizer.next();
                        Expression::Declaration {
                            identifier: identifier.get_value()
                        }
                    },
                    _ => throw(ExpectedIdentifier),
                }
            },
            _ => throw(ExpectedDeclaration (token.get_value())),
        }
    }
}