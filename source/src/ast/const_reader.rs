use crate::ast::{ConstAstNode
    , ConstReader, Value};

impl ConstReader {
    /*
     * εεΊιε
     * */
    pub fn read(node: Box<ConstAstNode>) -> Value {
        node.make_value()
    }

    pub fn new() -> Self {
        Self {
        }
    }
}

