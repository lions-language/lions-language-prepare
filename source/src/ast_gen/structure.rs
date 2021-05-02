struct Structure {
    memthods: FunctionSet
}

impl Structure {
    pub fn new() -> Self {
        Self {
            memthods: FunctionSet::new()
        }
    }
}

