use crate::lexer::{Literal, Type};
use crate::parser::ASTNode;
#[derive(Clone, Debug)]
pub enum ICInstruction {
    Literal(Literal),
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
    Import {
        module: String,
        name: String,
    },
}

pub struct Generator {
    ic: Vec<ICInstruction>,
    ast: Vec<ASTNode>,
    pos: usize,
}

impl Generator {
    pub fn new(ast: Vec<ASTNode>) -> Generator {
        let ic = Vec::new();
        let pos = 0;
        Generator { ic, ast, pos }
    }
    pub fn generate(&mut self) -> Vec<ICInstruction> {
        while self.generate_one_ic() {}
        self.ic.clone()
    }
    fn generate_one_ic(&mut self) -> bool {
        if self.pos == self.ast.len() {
            return false;
        }
        let ast_node = &self.ast[self.pos];
        self.pos += 1;
        match ast_node {
            ASTNode::Literal(lit) => self.ic.push(ICInstruction::Literal(lit.clone())),
            ASTNode::FnCall { function, args } => {
                self.ic.push(ICInstruction::FnCall {
                    function: function.clone(),
                    args: args.clone(),
                });
            }
            ASTNode::FnDecl {
                name,
                args,
                body,
                return_type,
            } => self.ic.push(ICInstruction::FnDecl {
                name: name.clone(),
                args: args.clone(),
                body: body.clone(),
                return_type: return_type.clone(),
            }),
            ASTNode::Import { module, name } => {
                self.ic.push(ICInstruction::Import {
                    module: module.clone(),
                    name: name.clone(),
                });
            }
        }
        true
    }
}

pub fn generate(ast: Vec<ASTNode>) -> Vec<ICInstruction> {
    Generator::new(ast).generate()
}
