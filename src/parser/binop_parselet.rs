//! Provides a parselet for binary operations.


use crate::parser::{
    Tokenizer,
    Token,
    TokenType,
    Expression,
    Parser,
    InfixParselet,
};


pub struct BinOpParselet;

impl InfixParselet for BinOpParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, left: Expression, token: Token) -> Expression {
        let right: Expression = match parser.parse(tokenizer) {
            Some(v) => v,
            None => todo!(),
        };

        match token.get_type() {
            TokenType::Plus
            | TokenType::Minus
            | TokenType::Multiply
            | TokenType::Divide
            | TokenType::Greater
            | TokenType::Lesser => {
                Expression::BinOp {
                    left: Box::new(left),
                    op: token.get_type(),
                    right: Box::new(right),
                }
            },
            _ => todo!()
        }
    }
}