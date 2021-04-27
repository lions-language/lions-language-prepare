use crate::ast::{AstNode, ConstAstNode, SingleOperatorAstNode
    , BinaryOperatorAstNode};
use crate::token::{TokenType};

enum PackageStr {
    Inner,
    Local,
    Other(String)
}

enum ModuleStr {
    Inner,
    Named(String)
}

struct Type {
    package_str: PackageStr,
    moduler_str: ModuleStr,
    typ_str: String
}

struct NodeData {
    typ: Type
}

struct Generator {
}

impl Generator {
    pub fn generate(self, node: AstNode) {
        use AstNode::*;
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
        let context = node.token.context();
        match context.typ {
            TokenType::U32(value) => {
                return NodeData{
                    typ: Type{
                        package_str: PackageStr::Inner,
                        moduler_str: ModuleStr::Inner,
                        typ_str: String::from("u32"),
                    }
                };
            },
            _ => {
                unimplemented!();
            }
        }
    }

    fn gen_single_operate(self, node: Box<SingleOperatorAstNode>) -> NodeData {
        unimplemented!();
    }

    fn gen_binary_operator(self, node: Box<BinaryOperatorAstNode>) -> NodeData {
        /*
         * 生成指令
         * 1. 判断左边和右边的类型
         * 2. 生成函数调用的指令
         *  - 从栈中读取几个参数
         *  - 返回值存放在哪里
         * */
        unimplemented!();
    }

    pub fn new() -> Self {
        Self {
        }
    }
}

