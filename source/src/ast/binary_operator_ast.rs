use crate::ast::{BinaryOperatorAstNode
    , AstNode};
use crate::token::TokenBox;

impl BinaryOperatorAstNode {
    pub fn token_ref(&self) -> &TokenBox {
        &self.token
    }

    pub fn new_branch(left: AstNode
        , right: AstNode
        , token: TokenBox) -> Self {
        Self {
            token: token,
            left: left,
            right: right,
        }
    }
}

