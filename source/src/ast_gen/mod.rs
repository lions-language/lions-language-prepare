use crate::ast::{AstNode};

struct Generator {
    root: AstNode
}

impl Generator {
    pub fn generate(self) {
        use AstNod::*;
        match self.root {
            SingleOperator(_) => {
            },
            BinaryOperator(_) => {
            },
            Variant(_) => {
            },
            FuncCall(_) => {
            },
            Const(node) => {
                self.gen_const(node);
            }
        }
    }

    fn gen_const(self, node: Box<ConstAstNode>) {
        /*
         *
         * */
    }

    pub fn new(node: AstNode) -> Self {
        Self {
        }
    }
}

