use crate::token::TokenBox;
use std::collections::VecDeque;

pub enum AstNode {
    SingleOperator(Box<SingleOperatorAstNode>),
    BinaryOperator(Box<BinaryOperatorAstNode>),
    Variant(Box<VariantAstNode>),
    FuncCall(Box<FuncCallAstNode>),
    Const(Box<ConstAstNode>)
}

pub struct SingleOperatorAstNode {
    token: TokenBox,
    child: AstNode
}

pub struct BinaryOperatorAstNode {
    token: TokenBox,
    left: AstNode,
    right: AstNode
}

pub struct VariantAstNode {
    // exp: new
    token: TokenBox,
    // exp: obj
    prefix: Option<TokenBox>,
    // exp: .
    opt: Option<TokenBox>
}

pub struct ConstAstNode {
    token: TokenBox
}

pub struct Type {
}

pub struct TypePrefix {
}

pub struct FuncCallParam {
    name: String,
    typ_prefix: TypePrefix,
    typ: Type,
}

pub struct FuncCallReturn {
}

pub struct FuncCallAstNode {
    prefix: Option<TokenBox>,
    token: Box<TokenBox>,
    params: Option<Vec<FuncCallParam>>,
    ret: Option<FuncCallReturn>
}

pub struct Reader {
}

pub struct SingleOperatorReader {
}

pub struct BinaryOperatorReader {
}

pub struct VariantReader {
}

pub struct ConstReader {
}

pub trait ValueExecuter {
    fn execute_single(self: Box<Self>, _opt_token: TokenBox) -> Value {
        unimplemented!();
    }
    fn execute_binary(self: Box<Self>, _opt_token: TokenBox, _right: Value) -> Value {
        unimplemented!();
    }
}

pub type Value = Box<dyn ValueExecuter>;

mod single_operator_ast;
mod binary_operator_ast;
mod const_ast;
mod single_operator_reader;
mod binary_operator_reader;
mod const_reader;
mod reader;
