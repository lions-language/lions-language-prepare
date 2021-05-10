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
    pub fn find(&self, typ_str: String) -> Option<&Structure> {
        self.structures.get(&typ_str)
    }

    pub fn new() -> Self {
        Self {
            structures: HashMap::new()
        }
    }
}

struct StructureSet {
    modules: HashMap<ast_gen::ModuleStr, StructureModule>
}

impl StructureSet {
    pub fn find(&self, module_str: ast_gen::ModuleStr, typ_str: String) -> Option<&Structure> {
        let module = match self.modules.get(&module_str) {
            Some(m) => m,
            None => {
                return None;
            }
        };
        module.find(typ_str)
    }

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
    pub fn find(&mut self, typ: ast_gen::Type) -> Option<&Structure> {
        use ast_gen::PackageStr::*;
        match typ.package_str {
            Inner => {
                self.inner.find(typ.moduler_str, typ.typ_str)
            },
            Local => {
                unimplemented!();
            },
            Other(v) => {
                unimplemented!();
            }
        }
    }

    pub fn new(set: StructureSet) -> Self {
        Self {
            inner: set
        }
    }
}

