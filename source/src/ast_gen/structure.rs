use std::co;llections::HashMap;

struct Structure {
    name: String,
    memthods: FunctionSet
}

impl Structure {
    pub fn find_method(&self, name: &str) {
    }

    pub fn new() -> Self {
        Self {
            name: String::new(),
            memthods: FunctionSet::new()
        }
    }
}

struct StructureSet {
    structures: HashMap<String, Structure>
}

impl StructureSet {
    pub fn new() -> Self {
        Self {
            structures: HashMap::new()
        }
    }
}

