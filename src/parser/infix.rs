//! Provides an interface for infix parselets


use crate::tokenizer::{
    Tokenizer,
    Token,
};

use crate::parser::{
    Parser,
    Expression,
};


pub trait InfixParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, left: Expression, token: Token) -> Expression;
}