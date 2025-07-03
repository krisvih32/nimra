use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq, Clone, Debug)]
pub enum Literal {
    String(String),
    Number(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Void,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Literal(Literal),
    Semicolon,
    Import,
    From,
    Type(Type),
    Fn,
    Identifier(String),
    OpenBrace,
    CloseBrace,
    Unknown(String),
    CloseParen,
    OpenParen,
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
            tokens: Vec::new(),
        }
    }

    fn push(&mut self, token: Token) {
        self.tokens.push(token);
        self.chars.next();
    }

    pub fn lex(mut self) -> Vec<Token> {
        while let Some(&ch) = self.chars.peek() {
            match ch {
                '(' => self.push(Token::OpenParen),
                ')' => self.push(Token::CloseParen),
                ';' => self.push(Token::Semicolon),
                '{' => self.push(Token::OpenBrace),
                '}' => self.push(Token::CloseBrace),
                '"' => {
                    self.chars.next(); // skip opening quote
                    let mut s = String::new();
                    while let Some(&c) = self.chars.peek() {
                        if c == '"' {
                            self.chars.next(); // skip closing quote
                            break;
                        }
                        s.push(c);
                        self.chars.next();
                    }
                    self.tokens.push(Token::Literal(Literal::String(s)));
                }
                c if c.is_ascii_digit() => {
                    let mut num = String::new();
                    while let Some(&c) = self.chars.peek() {
                        if c.is_ascii_digit() {
                            num.push(c);
                            self.chars.next();
                        } else {
                            break;
                        }
                    }
                    if let Ok(n) = num.parse() {
                        self.tokens.push(Token::Literal(Literal::Number(n)));
                    } else {
                        self.tokens.push(Token::Unknown(num));
                    }
                }
                c if c.is_ascii_alphabetic() || c == '_' => {
                    let mut ident = String::new();
                    while let Some(&c) = self.chars.peek() {
                        if c.is_ascii_alphanumeric() || c == '_' {
                            ident.push(c);
                            self.chars.next();
                        } else {
                            break;
                        }
                    }
                    let token = match ident.as_str() {
                        "import" => Token::Import,
                        "from" => Token::From,
                        "void" => Token::Type(Type::Void),
                        "fn" => Token::Fn,
                        _ => Token::Identifier(ident),
                    };
                    self.tokens.push(token);
                }
                c if c.is_whitespace() => {
                    self.chars.next();
                }
                _ => {
                    let mut unknown = String::new();
                    unknown.push(ch);
                    self.chars.next();
                    self.tokens.push(Token::Unknown(unknown));
                }
            }
        }
        self.tokens
    }
}

pub fn lex(input: &str) -> Vec<Token> {
    Lexer::new(input).lex()
}
