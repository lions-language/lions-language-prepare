use crate::ast_gen;
use std::collections::HashMap;

struct FunctionParam {
}

struct FunctionReturn {
}

struct Function {
    params: Vec<FunctionParam>,
    ret: Option<FunctionReturn>,
    ins: Vec<ast_gen::Instructure>
}

struct FunctionSet {
    functions: HashMap<String, Function>
}

impl FunctionSet {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new()
        }
    }
}

mod function;
