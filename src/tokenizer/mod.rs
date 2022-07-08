//! A simple tokenizer for Lolcode.


/// Provides a character stream for the tokenizer.
pub struct CharStream {
    stream: Vec<char>,
    index: usize,
}

impl CharStream {
    /// Creates a character stream from a `String`.
    pub fn new(string: String) -> Self {
        Self {
            stream: string.chars().collect::<Vec<char>>(),
            index: 0,
        }
    }

    /// Gets the next value from the character stream.
    pub fn next(&mut self) -> char {
        let chr = self.peek();
        self.index += 1;
        chr
    }

    /// Peeks at the next value from the character stream (without advancing the stream).
    pub fn peek(&self) -> char {
        if self.index >= self.stream.len() {
            '\0'
        } else {
            self.stream[self.index]
        }
    }
}


/// Defines the types of tokens available to the tokenizer.
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum TokenType {
    BeginPgrm,
    Include,
    EndPgrm,
    Identifier,
    Declaration,
    Assignment,
    Int,
    BeginLoop,
    EndLoop,
    Break,
    Plus,
    Minus,
    Multiply,
    Divide,
    Greater,
    Lesser,
    Equal,
    Call,
    EndCall,
    If,
    EndIf,
    Unknown,
    Eof,
}


/// Creates a token.
#[derive(Clone, Debug)]
pub struct Token {
    tokentype: TokenType,
    value: String,
}

/// Provides functions for tokens.
impl Token {
    pub fn new(v: String, t: TokenType) -> Self {
        Self {
            tokentype: t,
            value: v,
        }
    }

    pub fn get_type(&self) -> TokenType {
        self.tokentype
    }

    pub fn get_value(&self) -> String {
        self.value.to_owned()
    }
}


const SEPARATORS: &str = "\n\0? ";


/// Creates an abstraction over a tokenizer.
pub struct Tokenizer {
    stream: Vec<Token>,
    index: usize,
}

impl Tokenizer {
    /// Constructs a tokenizer from a `String`.
    pub fn new(string: String) -> Self {
        let mut charstream = CharStream::new(string);

        let mut stream: Vec<Token> = Vec::new();

        while let Some(t) = Self::get_next(&mut charstream) {
            stream.push(t);
        }
        
        Self {
            stream,
            index: 0,
        }
    }

    /// Gets all tokens out of the token stream.
    pub fn collect(&self) -> Vec<Token> {
        self.stream.to_owned()
    }

    /// Skips whitespace and comments in the character stream.
    fn skip_whitespace(charstream: &mut CharStream) {
        while charstream.peek() != '\0' {
            if SEPARATORS.contains(charstream.peek()) {
                charstream.next();
            } else {
                break;
            }
        }
    }

    /// Gets the next token from the character stream.
    fn get_next(charstream: &mut CharStream) -> Option<Token> {
        Self::skip_whitespace(charstream);

        let chr = charstream.next();

        let token = match chr {
            '\0' => return None,
            'A'..='z' | '?' => {
                let mut current = chr.to_string();

                while !SEPARATORS.contains(charstream.peek()) {
                    current.push(charstream.next());
                }

                match current.as_str() {
                    "I" => {
                        let mut c = current;

                        // This is the beginning of a loop instruction
                        while c.len() < 4 {
                            c.push(charstream.next());
                        }

                        if c.as_str() == "I IZ" {
                            Token::new(c, TokenType::Call)
                        } else {
                            // This is the beginning of a declaration
                            while c.len() < 7 {
                                c.push(charstream.next());
                            }

                            if c.as_str() == "I HAS A" {
                                Token::new(c, TokenType::Declaration)
                            } else {
                                Token::new(c, TokenType::Unknown)
                            }
                        }
                    },
                    "R" => Token::new(current, TokenType::Assignment),
                    "IM" => {
                        let mut c = current;

                        // This is the beginning of a loop instruction
                        while c.len() < 8 {
                            c.push(charstream.next());
                        }

                        if c.as_str() == "IM IN YR" {
                            Token::new(c, TokenType::BeginLoop)
                        } else {
                            // This is the beginning of a declaration
                            while c.len() < 11 {
                                c.push(charstream.next());
                            }

                            if c.as_str() == "IM OUTTA YR" {
                                Token::new(c, TokenType::EndLoop)
                            } else {
                                Token::new(c, TokenType::Unknown)
                            }
                        }
                    },
                    "IZ" => Token::new(current, TokenType::If),
                    "CAN" => {
                        let mut c = current;

                        // This is the beginning of a declaration
                        while c.len() < 7 {
                            c.push(charstream.next());
                        }

                        if c.as_str() == "CAN HAS" {
                            Token::new(c, TokenType::Include)
                        } else {
                            Token::new(c, TokenType::Unknown)
                        }
                    },
                    "BIGGER" => {
                        let mut c = current;

                        // This is the beginning of a comparison
                        while c.len() < 11 {
                            c.push(charstream.next());
                        }

                        if c.as_str() == "BIGGER THAN" {
                            Token::new(c, TokenType::Greater)
                        } else {
                            Token::new(c, TokenType::Unknown)
                        }
                    },
                    "SMALLER" => {
                        let mut c = current;

                        // This is the beginning of a comparison
                        while c.len() < 12 {
                            c.push(charstream.next());
                        }

                        if c.as_str() == "SMALLER THAN" {
                            Token::new(c, TokenType::Lesser)
                        } else {
                            Token::new(c, TokenType::Unknown)
                        }
                    },
                    "EQUAL" => {
                        let mut c = current;

                        // This is the beginning of a comparison
                        while c.len() < 8 {
                            c.push(charstream.next());
                        }

                        if c.as_str() == "EQUAL TO" {
                            Token::new(c, TokenType::Equal)
                        } else {
                            Token::new(c, TokenType::Unknown)
                        }
                    },
                    "ENUF" => Token::new(current, TokenType::Break),
                    "MKAY" => Token::new(current, TokenType::EndCall),
                    "KTHX" => Token::new(current, TokenType::EndIf),
                    "HAI" => Token::new(current, TokenType::BeginPgrm),
                    "KTHXBYE" => Token::new(current, TokenType::EndPgrm),
                    "BTW" => {
                        // This is the beginning of an inline comment
                        while charstream.peek() != '\0' && charstream.peek() != '\n' {
                            charstream.next();
                        }

                        match Self::get_next(charstream) {
                            Some(t) => t,
                            None => Token::new(String::new(), TokenType::Unknown),
                        }
                    },
                    "AN" => {
                        match Self::get_next(charstream) {
                            Some(t) => t,
                            None => Token::new(String::new(), TokenType::Unknown),
                        }
                    },
                    "OBTW" => {
                        // This is the beginning of a block comment
                        let mut c = String::new();

                        while charstream.peek() != '\0' {
                            if SEPARATORS.contains(charstream.peek()) {
                                match c.as_str() {
                                    "KTHX" => break,
                                    _ => {
                                        c = String::new();
                                        charstream.next();
                                    },
                                }
                            } else {
                                c.push(charstream.next());
                            }
                        }

                        match Self::get_next(charstream) {
                            Some(t) => t,
                            None => Token::new(String::new(), TokenType::Unknown),
                        }
                    },
                    _ => Token::new(current, TokenType::Identifier)
                }
            },
            '0'..='9' => {
                let mut current = chr.to_string();

                while !SEPARATORS.contains(charstream.peek()) {
                    current.push(charstream.next());
                }

                if let Ok(_) = str::parse::<i32>(&current) {
                    Token::new(current, TokenType::Int)
                } else {
                    return None;
                }
            },
            '+' => Token::new(chr.to_string(), TokenType::Plus),
            '-' => Token::new(chr.to_string(), TokenType::Minus),
            '*' => Token::new(chr.to_string(), TokenType::Multiply),
            '/' => Token::new(chr.to_string(), TokenType::Divide),
            _ => Token::new(chr.to_string(), TokenType::Unknown),
        };

        Some(token)
    }

    /// Peeks at the next token in the token stream without consuming it.
    pub fn peek(&self) -> Option<Token> {
        if self.index >= self.stream.len() {
            None
        } else {
            Some(self.stream[self.index].to_owned())
        }
    }

    /// Gets the next token out of the token stream.
    pub fn next(&mut self) -> Option<Token> {
        let token = self.peek();
        self.index += 1;
        token
    }
}