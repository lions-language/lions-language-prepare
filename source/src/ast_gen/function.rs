use std::collections::HashMap;
use crate::ast_gen;

struct FunctionParam {
}

struct FunctionReturn {
}

struct Function {
    params: Vec<FunctionParam>,
    ret: Option<FunctionReturn>,
    ins: Vec<ast_gen::Instructure>
}

pub struct FunctionSet {
    functions: HashMap<String, Function>
}

impl FunctionSet {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new()
        }
    }
}

