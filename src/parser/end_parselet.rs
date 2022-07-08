//! Provides a parselet for EndPgrm.


use crate::parser::{
    Tokenizer,
    Token,
    TokenType,
    Expression,
    Parser,
    PrefixParselet,
};


pub struct EndParselet;

impl PrefixParselet for EndParselet {
    fn parse(&self, _parser: &Parser, _tokenizer: &mut Tokenizer, token: Token) -> Expression {
        match token.get_type() {
            TokenType::EndPgrm => Expression::EndPgrm,
            _ => todo!(),
        }
    }
}