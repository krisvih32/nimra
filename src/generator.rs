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

    pub fn generate(&mut self) -> Result<Vec<ICInstruction>, String> {
        while self.generate_one_ic()? {}
        Ok(self.ic.clone())
    }

    fn generate_one_ic(&mut self) -> Result<bool, String> {
        if self.pos == self.ast.len() {
            return Ok(false);
        }
        let ast_node = self
            .ast
            .get(self.pos)
            .ok_or_else(|| format!("AST index {} out of bounds", self.pos))?;
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
        Ok(true)
    }
}

pub fn generate(ast: Vec<ASTNode>) -> Result<Vec<ICInstruction>, String> {
    Generator::new(ast).generate()
}
