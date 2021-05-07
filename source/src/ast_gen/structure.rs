use std::collections::HashMap;
use crate::ast_gen::{self, function};

struct Structure {
    name: String,
    memthods: function::FunctionSet
}

impl Structure {
    pub fn find_method(&self, name: &str) {
    }

    pub fn new() -> Self {
        Self {
            name: String::new(),
            memthods: function::FunctionSet::new()
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

pub struct StructureControl {
}

impl StructureControl {
    pub fn find(&mut self, typ: ast_gen::Type) {
        use ast_gen::PackageStr::*;
        match typ.package_str {
            Inner => {
            },
            Local => {
            },
            Other(v) => {
            }
        }
    }

    pub fn new() -> Self {
        Self {
        }
    }
}

