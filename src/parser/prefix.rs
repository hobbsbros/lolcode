//! Provides an interface for prefix parselets.


use crate::tokenizer::{
    Tokenizer,
    Token,
};

use crate::parser::{
    Parser,
    Expression,
};


pub trait PrefixParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression;
}