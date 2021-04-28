use crate::ast::{AstNode};
use crate::token::{Token, TokenType
    , TokenBox, TokenContext
    , NupResult, LedResult};
use crate::expression::{Expression};

struct PrefixPlusPlusToken {
    context: TokenContext
}

impl Token for PrefixPlusPlusToken {
    fn nup(self: Box<Self>, expr: &mut Expression) -> Result<AstNode, NupResult> {
        /*
         * TODO: 判断下一个是什么, 并调用下一个 token 的nup方法
         * */
        let next = match expr.take_next_one() {
            Some(t) => t,
            None => {
                return Err(NupResult::Error);
            }
        };
        next.nup(expr)
    }

    fn led(self: Box<Self>, _expr: &mut Expression, _left: Option<AstNode>) -> Result<AstNode, LedResult> {
        Err(LedResult::Null)
    }

    fn context_ref(&self) -> &TokenContext {
        &self.context
    }

    fn context(self: Box<Self>) -> TokenContext {
        self.context
    }
}

impl PrefixPlusPlusToken {
    fn new() -> Self {
        Self {
            context: TokenContext {
                typ: TokenType::PrefixPlusPlus,
                bp: 100
            }
        }
    }
}

pub fn make_prefix_plus_plus_token() -> TokenBox {
    Box::new(PrefixPlusPlusToken::new())
}


