use std::collections::VecDeque;
use crate::token;

pub struct Grammar {
    tokens: VecDeque<token::Token>
}

impl Grammar {
    pub fn lookup_next_n(&self, n: usize) -> Option<&token::Token> {
        if n > self.tokens.len() {
            return None;
        }
        self.tokens.get(n - 1)
    }

    pub fn lookup_next_one(&self) -> Option<&token::Token> {
        self.lookup_next_n(1)
    }

    pub fn take_next_one(&mut self) -> Option<token::Token> {
        if self.tokens.len() < 1 {
            return None;
        }
        self.tokens.pop_front()
    }

    pub fn skip_next_n(&mut self, n: usize) {
        if n > self.tokens.len() {
            return;
        }
        for _ in 0..n {
            self.tokens.pop_front();
        }
    }

    pub fn skip_next_one(&mut self) {
        self.skip_next_n(1);
    }

    pub fn new(tokens: VecDeque<token::Token>) -> Self {
        Self {
            tokens: VecDeque::new()
        }
    }
}

