use crate::ast::{AstNode, BinaryOperatorAstNode};
use crate::token::{Token, TokenType
    , TokenBox, TokenContext
    , NupResult, LedResult};
use crate::expression::{Expression};

struct PlusToken {
    context: TokenContext
}

impl Token for PlusToken {
    fn nup(self: Box<Self>, _expr: &mut Expression) -> Result<AstNode, NupResult> {
        Err(NupResult::Null)
    }

    fn led(self: Box<Self>, expr: &mut Expression, left: Option<AstNode>) -> Result<AstNode, LedResult> {
        let right = match expr.expression(&self.context.bp) {
            Ok(node) => node,
            Err(_) => {
                return Err(LedResult::Error);
            }
        };
        Ok(AstNode::BinaryOperator(Box::new(BinaryOperatorAstNode::new_branch(left.unwrap(), right, self))))
    }

    fn context_ref(&self) -> &TokenContext {
        &self.context
    }

    fn context(self: Box<Self>) -> TokenContext {
        self.context
    }
}

impl PlusToken {
    fn new() -> Self {
        Self {
            context: TokenContext {
                typ: TokenType::Plus,
                bp: 10
            }
        }
    }
}

pub fn make_plus_token() -> TokenBox {
    Box::new(PlusToken::new())
}


