use crate::ast::{ConstAstNode
    , ConstReader, Value};

impl ConstReader {
    /*
     * 后序遍历
     * */
    pub fn read(node: Box<ConstAstNode>) -> Value {
        node.make_value()
    }

    pub fn new() -> Self {
        Self {
        }
    }
}

