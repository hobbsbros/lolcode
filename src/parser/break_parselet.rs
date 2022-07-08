//! Provides a parselet for break statements.


use crate::parser::{
    Tokenizer,
    Token,
    Expression,
    Parser,
    PrefixParselet,
};


pub struct BreakParselet;

impl PrefixParselet for BreakParselet {
    fn parse(&self, _parser: &Parser, _tokenizer: &mut Tokenizer, _token: Token) -> Expression {
        Expression::Break
    }
}