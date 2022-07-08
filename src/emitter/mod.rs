//! A simple C emitter for the Lolcode compiler.


use crate::parser::{
    Parser,
    TokenType,
    Expression,
    Tokenizer,
};

use chrono::{
    Datelike,
    Timelike,
    Local,
};

use crate::error::{
    Error::*,
    throw,
};


pub struct Emitter {
    include: String,
    code: String,
}

impl Emitter {
    /// Constructs a new emitter from a parser.
    pub fn new(tokenizer: &mut Tokenizer, parser: &Parser) -> Self {
        let mut include: String = String::new();
        let mut code: String = String::new();

        for expression in parser.parse_all(tokenizer) {
            match expression {
                Expression::Include {
                    module: _,
                } => {
                    let line = Self::emit(&expression);
                    include.push_str(&line);
                },
                Expression::Loop {
                    body: _,
                } | Expression::If {
                    condition: _,
                    body: _,
                } => {
                    let line = Self::emit(&expression);
                    code.push_str(&line);
                    code.push('\n');
                },
                _ => {
                    let line = Self::emit(&expression);
                    code.push_str(&line);
                    if line.len() != 0 {
                        code.push(';');
                    }
                    code.push('\n');
                }
            };
        }

        Self {
            include,
            code,
        }
    }

    /// Gets a C module name from the Lolcode module name.
    fn get_module(module: String) -> String {
        match module.as_str() {
            "STDIO" => "<stdio.h>".to_string(),
            _ => throw(UnresolvedImport (module)),
        }
    }

    /// Gets an operation from the Lolcode operation.
    fn get_op(op: TokenType) -> String {
        let out = match op {
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Multiply => "*",
            TokenType::Divide => "/",
            TokenType::Greater => ">",
            TokenType::Lesser => "<",
            TokenType::Equal => "==",
            _ => throw(InvalidOperator),
        };

        out.to_string()
    }

    /// Converts an expression into code.
    fn emit(expr: &Expression) -> String {
        match expr {
            Expression::BeginPgrm => "".to_string(),
            Expression::Include {
                module: m,
            } => format!("#include {}\n", &Self::get_module(m.to_string())),
            Expression::Identifier (s) => s.to_string(),
            Expression::Int (i) => format!("{}", i),
            Expression::Declaration {
                identifier: i,
            } => format!("int {}", i),
            Expression::Assignment {
                identifier: i,
                value: v,
            } => format!("{} = {}", i, Self::emit(&*v)),
            Expression::BinOp {
                left: l,
                op: o,
                right: r,
            } => format!("({} {} {})", Self::emit(&*l), Self::get_op(*o), Self::emit(&*r)),
            Expression::Call {
                identifier: i,
                args: a,
            } => {
                let mut emitted = String::new();

                match i.as_str() {
                    "VISIBLE" => {
                        for (index, arg) in a.iter().enumerate() {
                            emitted.push_str("printf(\"%d\\n\", ");
                            emitted.push_str(&Self::emit(arg));
                            emitted.push_str(")");
                            if index < a.len() - 1 {
                                emitted.push_str(";\n");
                            }
                        }
                    },
                    _ => throw(UndeclaredIdentifier (i.to_string())),
                };

                emitted
            },
            Expression::Loop {
                body: b,
            } => {
                let mut emitted = String::new();

                emitted.push_str("while (1) {\n");

                for expr in b {
                    emitted.push_str(&Self::emit(&expr));
                    emitted.push_str(";\n");
                }

                emitted.push_str("}");

                emitted
            },
            Expression::Break => "break".to_string(),
            Expression::If {
                condition: c,
                body: b,
            } => {
                let mut emitted = String::new();

                emitted.push_str("if ");
                emitted.push_str(&Self::emit(c));
                emitted.push_str(" {\n");

                for expr in b {
                    emitted.push_str(&Self::emit(&expr));
                    emitted.push_str(";\n");
                }

                emitted.push_str("}");

                emitted
            },
            Expression::EndPgrm => "".to_string(),
        }
    }

    /// Gets the emitted code.
    pub fn output(&self) -> String {
        let now = Local::now();

        format!(
            "// Autogenerated by the Lolcode compiler on {:02}/{:02}/{:02} at {:02}:{:02}:{:02}\n{}\nint main(void) {{ \n{}\n}}\n",
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second(),
            &self.include,
            &self.code
        )
    }
}