use std::collections::HashMap;

struct Function {
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
