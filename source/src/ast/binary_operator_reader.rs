use crate::ast::{BinaryOperatorAstNode
    , BinaryOperatorReader, Value, Reader};

impl BinaryOperatorReader {
    /*
     * 后序遍历
     * */
    pub fn read(node: Box<BinaryOperatorAstNode>) -> Value {
        let left_value = Reader::read(node.left);
        let right_value = Reader::read(node.right);
        left_value.execute_binary(node.token, right_value)
    }

    pub fn new() -> Self {
        Self {
        }
    }
}

