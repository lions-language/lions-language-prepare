use crate::ast::{SingleOperatorAstNode
    , SingleOperatorReader, Value, Reader};

impl SingleOperatorReader {
    /*
     * 后序遍历
     * */
    pub fn read(node: Box<SingleOperatorAstNode>) -> Value {
        let v = Reader::read(node.child);
        v.execute_single(node.token)
    }

    pub fn new() -> Self {
        Self {
        }
    }
}

