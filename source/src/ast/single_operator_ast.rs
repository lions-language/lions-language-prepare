use crate::ast::{SingleOperatorAstNode
    , AstNode};
use crate::token::TokenBox;

impl SingleOperatorAstNode {
    pub fn token_ref(&self) -> &TokenBox {
        &self.token
    }

    pub fn new_branch(child: AstNode
        , token: TokenBox) -> Self {
        Self {
            token: token,
            child: child
        }
    }
}

