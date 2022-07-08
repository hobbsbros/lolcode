//! Provides precedence values.


use crate::parser::TokenType::{
    self,
    *
};


impl From<TokenType> for u8 {
    fn from(t: TokenType) -> u8 {
        match t {
            Assignment => 1,
            Plus => 3,
            Minus => 3,
            Multiply => 3,
            Divide => 3,
            Greater => 4,
            Lesser => 4,
            Equal => 4,
            _ => 0,
        }
    }
}