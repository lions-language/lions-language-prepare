use std::collections::VecDeque;
use crate::token;

pub struct Grammar {
    tokens: VecDeque<token::Token>
}

impl Grammar {
    pub fn new(tokens: VecDeque<token::Token>) -> Self {
        Self {
            tokens: VecDeque::new()
        }
    }
}

