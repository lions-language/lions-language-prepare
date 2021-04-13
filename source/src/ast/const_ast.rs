use crate::ast::{ConstAstNode, Value
    , ValueExecuter};
use crate::token::{TokenBox, TokenType};

struct U32Value {
    value: u32
}

impl ValueExecuter for U32Value {
    fn execute_single(mut self: Box<Self>, opt_token: TokenBox) -> Value {
        let typ = &opt_token.context_ref().typ;
        match typ {
            TokenType::PrefixPlusPlus => {
                self.value += 1;
            },
            _ => {
                unimplemented!("{:?}", typ);
            }
        }
        self
    }

    fn execute_binary(self: Box<Self>, _opt_token: TokenBox, _right: Value) -> Value {
        unimplemented!();
    }
}

impl U32Value {
    fn make_box(v: u32) -> Box<Self> {
        Box::new(Self {
            value: v
        })
    }
}

impl ConstAstNode {
    pub fn make_value(self: Box<Self>) -> Value {
        match self.token.context_ref().typ {
            TokenType::U32(v) => {
                U32Value::make_box(v)
            },
            _ => {
                unimplemented!();
            }
        }
    }

    pub fn token_ref(&self) -> &TokenBox {
        &self.token
    }

    pub fn new(token: TokenBox) -> Self {
        Self {
            token: token,
        }
    }
}

