use crate::ast::{AstNode, ConstAstNode, SingleOperatorAstNode
    , BinaryOperatorAstNode};
use crate::token::{TokenType};

const OPERATE_PLUS: &'static str = "plus";
const OPERATE_PREFIX_PLUS: &'static str = "prefix_plus";

enum Instructure {
    PushU32(u32)
}

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
    ins: Vec<Instructure>,
    structure_control: ast_gen::StructureControl,
}

impl Generator {
    pub fn generate(&mut self, node: AstNode) {
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

    fn gen_const(&mut self, node: Box<ConstAstNode>) -> NodeData {
        /*
         * 生成指令
         * 1. 将常量放入到 栈中
         * */
        let context = node.token.context();
        match context.typ {
            TokenType::U32(value) => {
                self.ins.push(Instructure::PushU32(value));
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

    fn gen_single_operate(&mut self, node: Box<SingleOperatorAstNode>) -> NodeData {
        let context = node.token.context();
        match context.typ {
            TokenType::PrefixPlusPlus => {
                /*
                 * 1. 获取 left 的 Type
                 * 2. 调用 left 的 prefix_plus_plus 函数, 并得到 NodeData
                 * */
                let left_node_data = self.generate(node.child);
                self.structure_control.find(typ);
                unimplemented!("");
            },
            _ => {
                unimplemented!("");
            }
        }
    }

    fn gen_binary_operator(&mut self, node: Box<BinaryOperatorAstNode>) -> NodeData {
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
            ins: Vec::new(),
            structure_control: ast_gen::structure::StructureControl::new(),
        }
    }
}

mod structure;
mod function;

