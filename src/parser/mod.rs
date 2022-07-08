//! A simple parser for the Lolcode compiler.


pub mod infix;
pub mod prefix;

pub mod identifier_parselet;
pub mod begin_parselet;
pub mod end_parselet;
pub mod declaration_parselet;
pub mod assignment_parselet;
pub mod binop_parselet;
pub mod include_parselet;
pub mod loop_parselet;
pub mod if_parselet;
pub mod literal_parselet;
pub mod break_parselet;
pub mod call_parselet;

use std::collections::HashMap;

pub use crate::tokenizer::{
    Token,
    TokenType,
    Tokenizer,
};

pub use infix::InfixParselet;
pub use prefix::PrefixParselet;

use identifier_parselet::IdentifierParselet;
use begin_parselet::BeginParselet;
use end_parselet::EndParselet;
use declaration_parselet::DeclarationParselet;
use assignment_parselet::AssignmentParselet;
use binop_parselet::BinOpParselet;
use include_parselet::IncludeParselet;
use loop_parselet::LoopParselet;
use if_parselet::IfParselet;
use literal_parselet::LiteralParselet;
use break_parselet::BreakParselet;
use call_parselet::CallParselet;


/// Enumerates the types of expression available to the parser.
#[derive(Debug)]
pub enum Expression {
    BeginPgrm,
    Include {
        module: String,
    },
    Identifier (String),
    Int (i32),
    Declaration {
        identifier: String,
    },
    Assignment {
        identifier: String,
        value: Box<Expression>,
    },
    BinOp {
        left: Box<Expression>,
        op: TokenType,
        right: Box<Expression>,
    },
    Call {
        identifier: String,
        args: Vec<Expression>,
    },
    Loop {
        body: Vec<Expression>,
    },
    Break,
    If {
        condition: Box<Expression>,
        body: Vec<Expression>,
    },
    EndPgrm,
}


/// Creates an abstraction over the Lolcode parser.
pub struct Parser {
    prefix_parselets: HashMap<TokenType, Box<dyn PrefixParselet>>,
    infix_parselets: HashMap<TokenType, Box<dyn InfixParselet>>,
}

impl Parser {
    /// Constructs a new parser.
    pub fn new() -> Self {
        let mut prefix_parselets: HashMap<TokenType, Box<dyn PrefixParselet>> = HashMap::new();
        let mut infix_parselets: HashMap<TokenType, Box<dyn InfixParselet>> = HashMap::new();

        // Declarative grammar begins here.
        prefix_parselets.insert(TokenType::Identifier, Box::new(IdentifierParselet {}));
        prefix_parselets.insert(TokenType::Break, Box::new(BreakParselet {}));
        prefix_parselets.insert(TokenType::Int, Box::new(LiteralParselet {}));
        prefix_parselets.insert(TokenType::BeginPgrm, Box::new(BeginParselet {}));
        prefix_parselets.insert(TokenType::EndPgrm, Box::new(EndParselet {}));
        prefix_parselets.insert(TokenType::Declaration, Box::new(DeclarationParselet {}));
        prefix_parselets.insert(TokenType::Include, Box::new(IncludeParselet {}));
        prefix_parselets.insert(TokenType::BeginLoop, Box::new(LoopParselet {}));
        prefix_parselets.insert(TokenType::If, Box::new(IfParselet {}));
        prefix_parselets.insert(TokenType::Call, Box::new(CallParselet {}));
        infix_parselets.insert(TokenType::Assignment, Box::new(AssignmentParselet {}));
        infix_parselets.insert(TokenType::Plus, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenType::Minus, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenType::Multiply, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenType::Divide, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenType::Greater, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenType::Lesser, Box::new(BinOpParselet {}));

        Self {
            prefix_parselets,
            infix_parselets,
        }
    }

    /// Parses from a tokenizer.
    pub fn parse(&self, tokenizer: &mut Tokenizer) -> Option<Expression> {
        let token = match tokenizer.next() {
            Some(t) => t,
            None => return None,
        };

        let parselet = match self.prefix_parselets.get(&token.get_type()) {
            Some(p) => p,
            None => todo!(),
        };

        let left: Expression = parselet.parse(self, tokenizer, token);

        let next = match tokenizer.peek() {
            Some(t) => t,
            None => return Some(left),
        };

        let parselet = match self.infix_parselets.get(&next.get_type()) {
            Some(p) => {
                tokenizer.next();
                p
            },
            None => return Some(left),
        };

        Some(parselet.parse(self, tokenizer, left, next))
    }

    /// Parses a program.
    pub fn parse_all(&self, tokenizer: &mut Tokenizer) -> Vec<Expression> {
        let mut program = Vec::new();

        while let Some(e) = self.parse(tokenizer) {
            program.push(e);
        }

        program
    }
}