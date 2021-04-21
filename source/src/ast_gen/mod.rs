use crate::ast::{AstNode};

enum Type {
}

struct NodeData {
    typ Type
}

struct Generator {
}

impl Generator {
    pub fn generate(self, node: AstNode) {
        use AstNod::*;
        match node {
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

    fn gen_const(self, node: Box<ConstAstNode>) -> NodeData {
        /*
         * 生成指令
         * 1. 将常量放入到 栈中
         * */
    }

    fn gen_binary_operator(self, node: Box<BinaryOperator>) -> NodeData {
        /*
         * 生成指令
         * 1. 判断左边和右边的类型
         * 2. 生成函数调用的指令
         *  - 从栈中读取几个参数
         *  - 返回值存放在哪里
         * */
    }

    pub fn new() -> Self {
        Self {
        }
    }
}

