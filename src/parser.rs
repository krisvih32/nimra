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
    parser.parse();
    parser.ast
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        let pos = 0;
        let ast = Vec::new();
        Parser { tokens, pos, ast }
    }

    pub fn parse(&mut self) -> &Vec<ASTNode> {
        while self.parse_token() {}
        &self.ast
    }

    fn parse_token(&mut self) -> bool {
        if let Some(token) = self.tokens.get(self.pos) {
            self.pos += 1;
            match token {
                Token::Literal(_) => {
                    panic!("Unexpected literal")
                }
                Token::Semicolon => {
                    // This should never happen
                    panic!("Unexpected semicolon")
                }
                Token::Import => {
                    let name: &Token = self.tokens.get(self.pos).expect("EOF");
                    if !matches!(*name, Token::Identifier(_)) {
                        panic!("Second word of import is not identifier");
                    }
                    self.pos += 1;
                    let from: &Token = self.tokens.get(self.pos).expect("EOF");
                    if !matches!(*from, Token::From) {
                        panic!("Third word of import is not from")
                    }
                    self.pos += 1;
                    let module: &Token = self.tokens.get(self.pos).expect("EOF");
                    if !matches!(*module, Token::Identifier(_)) {
                        panic!("Fourth word of import is not identifier")
                    }
                    self.pos += 1;
                    let semicolon: &Token = self.tokens.get(self.pos).unwrap();
                    if !matches!(semicolon, Token::Semicolon) {
                        panic!("Last character of import is not semicolon")
                    }
                    self.pos += 1;
                    // Construct AST Node

                    if let Token::Identifier(module) = module {
                        if let Token::Identifier(name) = name {
                            self.ast.push(ASTNode::Import {
                                module: module.to_string(),
                                name: name.to_string(),
                            })
                        }
                    }
                    true
                }
                Token::From => {
                    panic!("Unexpected from");
                }
                Token::Type(return_type) => {
                    // This is type, function decl
                    let fn_token = self.tokens.get(self.pos).expect("EOF");
                    if !matches!(fn_token, Token::Fn) {
                        panic!("fn expected after type")
                    }
                    self.pos += 1;
                    let fn_name = self.tokens.get(self.pos).expect("EOF");
                    if !matches!(fn_name, Token::Identifier(_)) {
                        panic!("Function name expected after fn")
                    }
                    self.pos += 1;
                    let open_paren = self.tokens.get(self.pos).expect("EOF");
                    if !matches!(open_paren, Token::OpenParen) {
                        panic!("Open paren expected after function name")
                    }
                    self.pos += 1;
                    let close_paren = self.tokens.get(self.pos).expect("EOF");
                    if !matches!(close_paren, Token::CloseParen) {
                        panic!("Close paren expected after open paren")
                    }
                    self.pos += 1;
                    let open_brace = self.tokens.get(self.pos).expect("EOF");
                    if !matches!(open_brace, Token::OpenBrace) {
                        panic!("Expected open brace")
                    }
                    self.pos += 1;
                    // We need to parse the function body
                    // Keep going until close brace and log everything
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
                        }
                    }
                    let fn_ast = parse(&fn_tokens);
                    // Make fn name string
                    if let Token::Identifier(fn_name_str) = fn_name {
                        self.ast.push(ASTNode::FnDecl {
                            name: fn_name_str.clone(),
                            args: Vec::new(),
                            body: fn_ast,
                            return_type: return_type.clone(),
                        });
                    }
                    true
                }

                Token::Fn => {
                    panic!("Unexpected fn")
                }
                Token::Identifier(ident) => {
                    // This is a call
                    let open_paren = self.tokens.get(self.pos).expect("EOF");
                    if !matches!(*open_paren, Token::OpenParen) {
                        panic!("Expected open paren")
                    }
                    self.pos += 1;
                    let close_paren_or_literal = self.tokens.get(self.pos).expect("EOF");
                    self.pos += 1;
                    if !matches!(*close_paren_or_literal, Token::CloseParen) {
                        let mut continue_execution = false;
                        if matches!(*close_paren_or_literal, Token::Literal(_)) {
                            // Make sure after that is close paren
                            let close_paren = self.tokens.get(self.pos).expect("EOF");
                            self.pos += 1;
                            if !matches!(*close_paren, Token::CloseParen) {
                                panic!("Expected close paren")
                            }
                            continue_execution = true
                        }
                        if !continue_execution {
                            panic!("Expected close paren or literal")
                        }
                    } else {
                        // For assurance, set close_paren_or_literal to the literal
                        let close_paren_or_literal: &Token =
                            self.tokens.get(self.pos).expect("EOF");
                        self.pos += 1;
                        if !matches!(*close_paren_or_literal, Token::Literal(_)) {
                            panic!("Expected literal")
                        }
                    }

                    // Next, we need semicolon
                    let semicolon = self.tokens.get(self.pos).expect("EOF");
                    self.pos += 1;
                    if !matches!(semicolon, Token::Semicolon) {
                        panic!("Expected semicolon");
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

                    true
                }
                Token::OpenBrace => {
                    panic!("Unexpected open brace")
                }
                Token::CloseBrace => {
                    panic!("Unexpected close brace")
                }
                Token::Unknown(_) => {
                    panic!("Syntax error")
                }
                Token::CloseParen => {
                    panic!("Unexpected close paren");
                }
                Token::OpenParen => {
                    panic!("Unexpected open paren");
                }
            }
        } else {
            false
        }
    }
}
