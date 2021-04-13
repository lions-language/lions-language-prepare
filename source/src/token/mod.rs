use crate::ast::{AstNode};
use crate::expression::Expression;
use std::collections::VecDeque;

pub enum NupResult {
    Error,
    Null
}

pub enum LedResult {
    Error,
    Null
}

#[derive(Debug)]
pub enum TokenType {
    U32(u32),
    Plus,
    PrefixPlusPlus
}

pub trait Token {
    fn nup(self: Box<Self>, expr: &mut Expression) -> Result<AstNode, NupResult>;
    fn led(self: Box<Self>, expr: &mut Expression, left: Option<AstNode>) -> Result<AstNode, LedResult>;
    fn context_ref(&self) -> &TokenContext;
}

pub struct TokenContext {
    pub typ: TokenType,
    pub bp: u8
}

pub type TokenBox = Box<dyn Token>;

pub struct TokenOption {
    tokens: VecDeque<TokenBox>,
    index: usize
}

impl TokenOption {
    pub fn lookup_next_n(&self, n: usize) -> Option<&TokenBox> {
        if n > self.tokens.len() {
            return None;
        }
        self.tokens.get(n - 1)
    }

    pub fn lookup_next_one(&self) -> Option<&TokenBox> {
        self.lookup_next_n(1)
    }

    pub fn take_next_one(&mut self) -> Option<TokenBox> {
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

    pub fn new(tokens: VecDeque<TokenBox>) -> Self {
        Self {
            tokens: tokens,
            index: 0
        }
    }
}

pub mod plus;
pub mod number;
pub mod prefix_plusplus;

