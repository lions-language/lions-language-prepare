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

struct StructureModule {
    structures: HashMap<String, Structure>
}

impl StructureModule {
    pub fn new() -> Self {
        Self {
            structures: HashMap::new()
        }
    }
}

struct StructureSet {
    modules: HashMap<String, StructureModule>
}

impl StructureSet {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new()
        }
    }
}

/////////////////////////////////////////
pub struct StructureControl {
    inner: StructureSet
}

impl StructureControl {
    pub fn find(&mut self, typ: ast_gen::Type) -> &StructureSet {
        use ast_gen::PackageStr::*;
        match typ.package_str {
            Inner => {
                self.inner.find(typ.module_str, typ.typ_str)
            },
            Local => {
                unimplemented!();
            },
            Other(v) => {
                unimplemented!();
            }
        }
    }

    pub fn new() -> Self {
        Self {
        }
    }
}

