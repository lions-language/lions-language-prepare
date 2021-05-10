use crate::token::{TokenBox};

pub struct Grammar {
    tokens: Vec<TokenBox>
}

impl Grammar {
    pub fn lookup_next_one(&self) -> Option<&TokenBox> {
        self.lookup_next_n(1)
    }

    pub fn lookup_next_n(&self, n: usize) -> Option<&TokenBox> {
        if n > self.tokens.len() {
            return None;
        }
        self.tokens.get(n)
    }

    pub fn new(tokens: Vec<TokenBox>) -> Self {
        Self {
            tokens: tokens
        }
    }
}

mod expression;
mod var_declare;

