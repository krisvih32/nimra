#[derive(Debug, PartialEq)]
pub enum Token {
    Semicolon,              
    Import,                 
    From,                   
    Void,                   
    Fn,                     
    Identifier(String),     
    Number(i64),            
    OpenBrace,              
    CloseBrace,             
    StringLiteral(String),  
    Unknown(String),
    CloseParen,
    OpenParen,       
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '(' => { tokens.push(Token::OpenParen); chars.next(); }
            ')' => { tokens.push(Token::CloseParen); chars.next(); }
            ';' => { tokens.push(Token::Semicolon); chars.next(); }
            '{' => { tokens.push(Token::OpenBrace); chars.next(); }
            '}' => { tokens.push(Token::CloseBrace); chars.next(); }
            '"' => {
                chars.next(); // skip opening quote
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    if c == '"' {
                        chars.next(); // skip closing quote
                        break;
                    }
                    s.push(c);
                    chars.next();
                }
                tokens.push(Token::StringLiteral(s));
            }
            c if c.is_ascii_digit() => {
                let mut num = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() {
                        num.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if let Ok(n) = num.parse() {
                    tokens.push(Token::Number(n));
                } else {
                    tokens.push(Token::Unknown(num));
                }
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let token = match ident.as_str() {
                    "import" => Token::Import,
                    "from" => Token::From,
                    "void" => Token::Void,
                    "fn" => Token::Fn,
                    _ => Token::Identifier(ident),
                };
                tokens.push(token);
            }
            c if c.is_whitespace() => { chars.next(); }
            _ => {
                let mut unknown = String::new();
                unknown.push(ch);
                chars.next();
                tokens.push(Token::Unknown(unknown));
            }
        }
    }
    tokens
}
