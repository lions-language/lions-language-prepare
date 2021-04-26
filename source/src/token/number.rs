use crate::ast::{AstNode, ConstAstNode};
use crate::token::{Token, TokenType
    , TokenBox, TokenContext
    , NupResult
    , LedResult};
use crate::expression::Expression;

struct U32Token {
    context: TokenContext
}

impl Token for U32Token {
    fn nup(self: Box<Self>, _expr: &mut Expression) -> Result<AstNode, NupResult> {
        // let token = expr.take_next_one().unwrap();
        Ok(AstNode::Const(Box::new(ConstAstNode::new(self))))
    }

    fn led(self: Box<Self>, _expr: &mut Expression
           , _left: Option<AstNode>) -> Result<AstNode, LedResult> {
        Err(LedResult::Null)
    }

    fn context_ref(&self) -> &TokenContext {
        &self.context
    }

    fn context(self) -> TokenContext {
        self.context
    }
}

impl U32Token {
    fn new(value: u32) -> Self {
        Self {
            context: TokenContext {
                typ: TokenType::U32(value),
                bp: 0
            }
        }
    }
}

pub fn make_u32_token(value: u32) -> TokenBox {
    Box::new(U32Token::new(value))
}

