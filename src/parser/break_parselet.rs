//! Provides a parselet for break statements.


use crate::parser::{
    Tokenizer,
    Token,
    TokenType,
    Expression,
    Parser,
    PrefixParselet,
};


pub struct BreakParselet;

impl PrefixParselet for BreakParselet {
    fn parse(&self, _parser: &Parser, _tokenizer: &mut Tokenizer, token: Token) -> Expression {
        match token.get_type() {
            TokenType::Break => Expression::Break,
            _ => todo!(),
        }
    }
}