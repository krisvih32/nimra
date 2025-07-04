use crate::lexer::{Literal, Token, Type};

#[derive(Debug, Clone)]
pub enum ASTNode {
    Literal(Literal),
    Import {
        module: String,
        name: String,
    },
    FnDecl {
        name: String,
        args: Vec<ASTNode>,
        body: Vec<ASTNode>,
        return_type: Type,
    },
    FnCall {
        function: String,
        args: Vec<ASTNode>,
    },
}

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    pos: usize,
    ast: Vec<ASTNode>,
}

pub fn parse(tokens: &Vec<Token>) -> Vec<ASTNode> {
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => ast.clone(),
        Err(e) => {
            eprintln!("Parse error: {e}");
            Vec::new()
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        let pos = 0;
        let ast = Vec::new();
        Parser { tokens, pos, ast }
    }

    pub fn parse(&mut self) -> Result<&Vec<ASTNode>, String> {
        while let Ok(true) = self.parse_token() {}
        Ok(&self.ast)
    }

    pub fn parse_token(&mut self) -> Result<bool, String> {
        if let Some(token) = self.tokens.get(self.pos) {
            self.pos += 1;
            match token {
                Token::Literal(_) => Err("Unexpected literal".to_string()),
                Token::Semicolon => Err("Unexpected semicolon".to_string()),
                Token::Import => {
                    let name: &Token = self
                        .tokens
                        .get(self.pos)
                        .ok_or("Unexpected EOF: expected identifier after import")?;
                    if !matches!(*name, Token::Identifier(_)) {
                        return Err("Second word of import is not identifier".to_string());
                    }
                    self.pos += 1;
                    let from: &Token = self
                        .tokens
                        .get(self.pos)
                        .ok_or("Unexpected EOF: expected 'from' after import identifier")?;
                    if !matches!(*from, Token::From) {
                        return Err("Third word of import is not from".to_string());
                    }
                    self.pos += 1;
                    let module: &Token = self
                        .tokens
                        .get(self.pos)
                        .ok_or("Unexpected EOF: expected module identifier after from")?;
                    if !matches!(*module, Token::Identifier(_)) {
                        return Err("Fourth word of import is not identifier".to_string());
                    }
                    self.pos += 1;
                    let semicolon: &Token = self
                        .tokens
                        .get(self.pos)
                        .ok_or("Unexpected EOF: expected semicolon after import statement")?;
                    if !matches!(semicolon, Token::Semicolon) {
                        return Err("Last character of import is not semicolon".to_string());
                    }
                    self.pos += 1;
                    if let Token::Identifier(module) = module {
                        if let Token::Identifier(name) = name {
                            self.ast.push(ASTNode::Import {
                                module: module.to_string(),
                                name: name.to_string(),
                            })
                        }
                    }
                    Ok(true)
                }
                Token::From => Err("Unexpected from".to_string()),
                Token::Type(return_type) => {
                    let fn_token = self
                        .tokens
                        .get(self.pos)
                        .ok_or("Unexpected EOF: expected fn after type")?;
                    if !matches!(fn_token, Token::Fn) {
                        return Err("fn expected after type".to_string());
                    }
                    self.pos += 1;
                    let fn_name = self
                        .tokens
                        .get(self.pos)
                        .ok_or("Unexpected EOF: expected function name after fn")?;
                    if !matches!(fn_name, Token::Identifier(_)) {
                        return Err("Function name expected after fn".to_string());
                    }
                    self.pos += 1;
                    let open_paren = self
                        .tokens
                        .get(self.pos)
                        .ok_or("Unexpected EOF: expected open paren after function name")?;
                    if !matches!(open_paren, Token::OpenParen) {
                        return Err("Open paren expected after function name".to_string());
                    }
                    self.pos += 1;
                    let close_paren = self
                        .tokens
                        .get(self.pos)
                        .ok_or("Unexpected EOF: expected close paren after open paren")?;
                    if !matches!(close_paren, Token::CloseParen) {
                        return Err("Close paren expected after open paren".to_string());
                    }
                    self.pos += 1;
                    let open_brace = self
                        .tokens
                        .get(self.pos)
                        .ok_or("Unexpected EOF: expected open brace after close paren")?;
                    if !matches!(open_brace, Token::OpenBrace) {
                        return Err("Expected open brace".to_string());
                    }
                    self.pos += 1;
                    let mut fn_tokens = Vec::new();
                    let mut close_brace_passed = false;
                    while !close_brace_passed {
                        if let Some(fn_token) = self.tokens.get(self.pos) {
                            self.pos += 1;
                            if *fn_token != Token::CloseBrace {
                                fn_tokens.push(fn_token.clone());
                            } else {
                                close_brace_passed = true
                            }
                        } else {
                            return Err(
                                "Unexpected EOF: expected close brace in function body".to_string()
                            );
                        }
                    }
                    let fn_ast = parse(&fn_tokens);
                    if let Token::Identifier(fn_name_str) = fn_name {
                        self.ast.push(ASTNode::FnDecl {
                            name: fn_name_str.clone(),
                            args: Vec::new(),
                            body: fn_ast,
                            return_type: return_type.clone(),
                        });
                    }
                    Ok(true)
                }
                Token::Fn => Err("Unexpected fn".to_string()),
                Token::Identifier(ident) => {
                    let open_paren = self
                        .tokens
                        .get(self.pos)
                        .ok_or("Unexpected EOF: expected open paren after identifier")?;
                    if !matches!(*open_paren, Token::OpenParen) {
                        return Err("Expected open paren".to_string());
                    }
                    self.pos += 1;
                    let close_paren_or_literal = self.tokens.get(self.pos).ok_or(
                        "Unexpected EOF: expected close paren or literal after open paren",
                    )?;
                    self.pos += 1;
                    if !matches!(*close_paren_or_literal, Token::CloseParen) {
                        let mut continue_execution = false;
                        if matches!(*close_paren_or_literal, Token::Literal(_)) {
                            let close_paren = self
                                .tokens
                                .get(self.pos)
                                .ok_or("Unexpected EOF: expected close paren after literal")?;
                            self.pos += 1;
                            if !matches!(*close_paren, Token::CloseParen) {
                                return Err("Expected close paren".to_string());
                            }
                            continue_execution = true
                        }
                        if !continue_execution {
                            return Err("Expected close paren or literal".to_string());
                        }
                    } else {
                        let close_paren_or_literal: &Token = self
                            .tokens
                            .get(self.pos)
                            .ok_or("Unexpected EOF: expected literal after close paren")?;
                        self.pos += 1;
                        if !matches!(*close_paren_or_literal, Token::Literal(_)) {
                            return Err("Expected literal".to_string());
                        }
                    }
                    let semicolon = self
                        .tokens
                        .get(self.pos)
                        .ok_or("Unexpected EOF: expected semicolon after function call")?;
                    self.pos += 1;
                    if !matches!(semicolon, Token::Semicolon) {
                        return Err("Expected semicolon".to_string());
                    }
                    if let Token::Literal(literal) = close_paren_or_literal {
                        match literal {
                            Literal::String(s) => {
                                self.ast.push(ASTNode::FnCall {
                                    function: ident.to_string(),
                                    args: vec![ASTNode::Literal(Literal::String(s.clone()))],
                                });
                            }
                            Literal::Number(n) => {
                                self.ast.push(ASTNode::FnCall {
                                    function: ident.to_string(),
                                    args: vec![ASTNode::Literal(Literal::Number(*n))],
                                });
                            }
                        }
                    }
                    Ok(true)
                }
                Token::OpenBrace => Err("Unexpected open brace".to_string()),
                Token::CloseBrace => Err("Unexpected close brace".to_string()),
                Token::Unknown(_) => Err("Syntax error".to_string()),
                Token::CloseParen => Err("Unexpected close paren".to_string()),
                Token::OpenParen => Err("Unexpected open paren".to_string()),
            }
        } else {
            Ok(false)
        }
    }
}
