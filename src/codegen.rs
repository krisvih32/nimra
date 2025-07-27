/*
 * Copyright (C) 2025 Vihaan Krishnan
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/agpl-3.0.html>.
 */

use crate::{
    generator::ICInstruction,
    lexer::{Literal, Type},
    parser::ASTNode,
};

pub struct CodeGen {
    ic: Vec<ICInstruction>,
    code: String,
    pos: usize,
    needs_stdlib: bool,
}

impl CodeGen {
    // Creates a new code generator using a vector of ICInstructions
    pub fn new(ic: Vec<ICInstruction>) -> Self {
        let needs_stdlib = ic.iter().any(contains_exit_call_ic);
        CodeGen {
            ic,
            code: String::new(),
            pos: 0,
            needs_stdlib,
        }
    }

    pub fn generate(&mut self) -> Result<String, String> {
        if self.needs_stdlib {
            self.code.push_str("#include <stdlib.h>\n");
        }
        while self.pos < self.ic.len() {
            self.code
                .push_str(&self.generate_instruction(&self.ic[self.pos])?);
            self.pos += 1;
        }
        Ok(self.code.clone())
    }

    fn generate_instruction(&self, ic: &ICInstruction) -> Result<String, String> {
        match ic {
            ICInstruction::Literal(lit) => Ok(format!("{};", self.literal_to_c(lit))),
            ICInstruction::Import { .. } => Ok(String::new()),
            ICInstruction::FnCall { function, args } => {
                if function == "return" {
                    let arg_str = if let Some(arg) = args.first() {
                        self.ast_node_expr(arg)
                    } else {
                        String::new()
                    };
                    Ok(format!("return {arg_str};"))
                } else if function == "exit" {
                    let arg_list = args
                        .iter()
                        .map(|arg| self.ast_node_expr(arg))
                        .collect::<Vec<_>>()
                        .join(", ");
                    Ok(format!("exit({arg_list});"))
                } else {
                    let arg_list = args
                        .iter()
                        .map(|arg| self.ast_node_expr(arg))
                        .collect::<Vec<_>>()
                        .join(", ");
                    Ok(format!("{function}({arg_list});"))
                }
            }
            ICInstruction::FnDecl {
                name,
                args,
                body,
                return_type,
            } => {
                let is_main = name == "main";
                let ret_type = if is_main {
                    "int"
                } else {
                    self.type_to_c(return_type)
                };
                let arg_list = if args.is_empty() {
                    "void".to_string()
                } else {
                    args.iter()
                        .map(|_| "void".to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                };
                let mut body_code = String::new();
                for stmt in body {
                    body_code.push_str(&self.ast_node_to_c(stmt)?);
                }
                if is_main && !has_explicit_return_or_exit(body) {
                    body_code.push_str("return 0;");
                }
                Ok(format!("{ret_type} {name}({arg_list}) {{{body_code}}}"))
            }
        }
    }

    fn ast_node_to_c(&self, node: &ASTNode) -> Result<String, String> {
        match node {
            ASTNode::Literal(lit) => Ok(format!("{};", self.literal_to_c(lit))),
            ASTNode::FnCall { function, args } => {
                if function == "return" {
                    let arg_str = if let Some(arg) = args.first() {
                        self.ast_node_expr(arg)
                    } else {
                        String::new()
                    };
                    Ok(format!("return {arg_str};"))
                } else if function == "exit" {
                    if let Some(ASTNode::Literal(Literal::Number(n))) = args.first() {
                        if !(*n >= 0 && *n <= 255) {
                            return Err("Exit should be between 0 and 255".to_string());
                        }
                    }
                    let arg_list = args
                        .iter()
                        .map(|arg| self.ast_node_expr(arg))
                        .collect::<Vec<_>>()
                        .join(", ");
                    Ok(format!("exit({arg_list});"))
                } else {
                    let arg_list = args
                        .iter()
                        .map(|arg| self.ast_node_expr(arg))
                        .collect::<Vec<_>>()
                        .join(", ");
                    Ok(format!("{function}({arg_list});"))
                }
            }
            ASTNode::Import { .. } => Ok(String::new()),
            ASTNode::FnDecl {
                name,
                args,
                body,
                return_type,
            } => {
                let is_main = name == "main";
                let ret_type = if is_main {
                    "int"
                } else {
                    self.type_to_c(return_type)
                };
                let arg_list = if args.is_empty() {
                    "void".to_string()
                } else {
                    args.iter()
                        .map(|_| "void".to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                };
                let mut body_code = String::new();
                for stmt in body {
                    body_code.push_str(&self.ast_node_to_c(stmt)?);
                }
                if is_main && !has_explicit_return_or_exit(body) {
                    body_code.push_str("return 0;");
                }
                Ok(format!("{ret_type} {name}({arg_list}) {{{body_code}}}"))
            }
        }
    }

    /// Generates expressions only (no trailing semicolon)
    fn ast_node_expr(&self, node: &ASTNode) -> String {
        match node {
            ASTNode::Literal(lit) => self.literal_to_c(lit),
            _ => String::new(),
        }
    }

    fn literal_to_c(&self, lit: &Literal) -> String {
        match lit {
            Literal::Number(n) => n.to_string(),
            Literal::String(s) => format!("\"{s}\""),
        }
    }

    fn type_to_c(&self, ty: &Type) -> &'static str {
        match ty {
            Type::Void => "void",
        }
    }
}

fn has_explicit_return_or_exit(stmts: &[ASTNode]) -> bool {
    for stmt in stmts {
        match stmt {
            ASTNode::FnCall { function, .. } if function == "return" || function == "exit" => {
                return true
            }
            ASTNode::FnDecl { body, .. } => {
                if has_explicit_return_or_exit(body) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn contains_exit_call_ic(ic: &ICInstruction) -> bool {
    match ic {
        ICInstruction::FnCall { function, .. } if function == "exit" => true,
        ICInstruction::FnDecl { body, .. } => body.iter().any(contains_exit_call_ast),
        _ => false,
    }
}
fn contains_exit_call_ast(ast: &ASTNode) -> bool {
    match ast {
        ASTNode::FnCall { function, .. } if function == "exit" => true,
        ASTNode::FnDecl { body, .. } => body.iter().any(contains_exit_call_ast),
        _ => false,
    }
}

pub fn codegen(ic: Vec<ICInstruction>) -> Result<String, String> {
    CodeGen::new(ic).generate()
}
