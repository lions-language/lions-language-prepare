use crate::ast::{AstNode, Reader
    , SingleOperatorReader, ConstReader
    , Value};

impl Reader {
    pub fn read(node: AstNode) -> Value {
        match node {
            AstNode::SingleOperator(n) => {
                SingleOperatorReader::read(n)
            },
            AstNode::BinaryOperator(_n) => {
                unimplemented!();
            },
            AstNode::Const(n) => {
                ConstReader::read(n)
            },
            AstNode::FuncCall(_) => {
                unimplemented!();
            }
            AstNode::Variant(_) => {
                unimplemented!();
            }
        }
    }
}

