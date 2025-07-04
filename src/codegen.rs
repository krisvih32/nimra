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
    pub fn new(ic: Vec<ICInstruction>) -> Self {
        let needs_stdlib = ic.iter().any(|ic| contains_exit_call_ic(ic));
        CodeGen {
            ic,
            code: String::new(),
            pos: 0,
            needs_stdlib,
        }
    }

    pub fn generate(&mut self) -> String {
        if self.needs_stdlib {
            self.code.push_str("#include <stdlib.h>\n");
        }
        while self.pos < self.ic.len() {
            self.code.push_str(&self.generate_instruction(&self.ic[self.pos]));
            self.pos += 1;
        }
        self.code.clone()
    }

    fn generate_instruction(&self, ic: &ICInstruction) -> String {
        match ic {
            ICInstruction::Literal(lit) => format!("{};", self.literal_to_c(lit)),
            ICInstruction::Import { .. } => String::new(),
            ICInstruction::FnCall { function, args } => {
                if function == "return" {
                    let arg_str = if let Some(arg) = args.get(0) {
                        self.ast_node_expr(arg)
                    } else {
                        String::new()
                    };
                    format!("return {};", arg_str)
                } else if function == "exit" {
                    let arg_list = args.iter()
                        .map(|arg| self.ast_node_expr(arg))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("exit({});", arg_list)
                } else {
                    let arg_list = args.iter()
                        .map(|arg| self.ast_node_expr(arg))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("{}({});", function, arg_list)
                }
            }
            ICInstruction::FnDecl { name, args, body, return_type } => {
                let is_main = name == "main";
                let ret_type = if is_main { "int" } else { self.type_to_c(return_type) };
                let arg_list = if args.is_empty() { "void".to_string() } else {
                    args.iter().map(|_| "void".to_string()).collect::<Vec<_>>().join(", ")
                };
                let mut body_code = String::new();
                for stmt in body {
                    body_code.push_str(&self.ast_node_to_c(stmt));
                }
                if is_main && !has_explicit_return_or_exit(body) {
                    body_code.push_str("return 0;");
                }
                format!("{} {}({}) {{{}}}", ret_type, name, arg_list, body_code)
            }
        }
    }

    fn ast_node_to_c(&self, node: &ASTNode) -> String {
        match node {
            ASTNode::Literal(lit) => format!("{};", self.literal_to_c(lit)),
            ASTNode::FnCall { function, args } => {
                if function == "return" {
                    let arg_str = if let Some(arg) = args.get(0) {
                        self.ast_node_expr(arg)
                    } else {
                        String::new()
                    };
                    format!("return {};", arg_str)
                } else if function == "exit" {
                    let arg_list = args.iter()
                        .map(|arg| self.ast_node_expr(arg))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("exit({});", arg_list)
                } else {
                    let arg_list = args.iter()
                        .map(|arg| self.ast_node_expr(arg))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("{}({});", function, arg_list)
                }
            }
            ASTNode::Import { .. } => String::new(),
            ASTNode::FnDecl { name, args, body, return_type } => {
                let is_main = name == "main";
                let ret_type = if is_main { "int" } else { self.type_to_c(return_type) };
                let arg_list = if args.is_empty() { "void".to_string() } else {
                    args.iter().map(|_| "void".to_string()).collect::<Vec<_>>().join(", ")
                };
                let mut body_code = String::new();
                for stmt in body {
                    body_code.push_str(&self.ast_node_to_c(stmt));
                }
                if is_main && !has_explicit_return_or_exit(body) {
                    body_code.push_str("return 0;");
                }
                format!("{} {}({}) {{{}}}", ret_type, name, arg_list, body_code)
            }
        }
    }

    /// Generates expressions only (no trailing semicolon)
    fn ast_node_expr(&self, node: &ASTNode) -> String {
        match node {
            ASTNode::Literal(lit) => self.literal_to_c(lit),
            // Extend for more expression types if needed
            _ => String::new(),
        }
    }

    fn literal_to_c(&self, lit: &Literal) -> String {
        match lit {
            Literal::Number(n) => n.to_string(),
            Literal::String(s) => format!("\"{}\"", s),
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
            ASTNode::FnCall { function, .. } if function == "return" || function == "exit" => return true,
            ASTNode::FnDecl { body, .. } => {
                if has_explicit_return_or_exit(body) { return true; }
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

pub fn codegen(ic: Vec<ICInstruction>) -> String {
    CodeGen::new(ic).generate()
}
