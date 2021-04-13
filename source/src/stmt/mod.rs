use crate::ast::AstNode;

pub struct ExpressionStmt {
    node: AstNode
}

pub struct BlockStmt {
    stmts: Vec<Box<Stmt>>
}

pub enum Stmt {
    Expression(ExpressionStmt),
    Block(BlockStmt)
}

mod reader;

