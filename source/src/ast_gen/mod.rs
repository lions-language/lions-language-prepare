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
         * 生成指令
         * 1. 将常量放入到 栈中
         * */
    }

    fn gen_binary_operator(self, node: Box<BinaryOperator>) {
        /*
         * 生成指令
         * 1. 判断左边和右边的类型
         * 2. 根据类型, 在字节码中指定读取多少字节
         * */
    }

    pub fn new(node: AstNode) -> Self {
        Self {
        }
    }
}

