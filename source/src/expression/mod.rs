use crate::token::{TokenOption, TokenBox};
use crate::ast::{AstNode};

#[derive(Debug)]
pub enum ExpressionResult {
    NupError,
    LedError,
    Error
}

pub struct Expression {
    token_option: TokenOption
}

impl Expression {
    pub fn expression(&mut self, operator_bp: &u8)
        -> Result<AstNode, ExpressionResult> {
        let input_token = match self.token_option.take_next_one() {
            Some(t) => {
                t
            },
            None => {
                return Err(ExpressionResult::Error);
            }
        };
        let mut node = match input_token.nup(self) {
            Ok(node) => {
                node
            },
            Err(_err) => {
                return Err(ExpressionResult::NupError);
            }
        };
        // println!("{:?}", node.token_ref().as_ref().unwrap().context_ref().typ);
        let mut next_token = match self.token_option.lookup_next_one() {
            Some(t) => {
                t
            },
            None => {
                // return Err(ExpressionResult::End);
                return Ok(node);
            }
        };
        while &next_token.context_ref().bp > operator_bp {
            let next = self.token_option.take_next_one().unwrap();
            // println!("{:?}", node.token_ref().as_ref().unwrap().context_ref().typ);
            node = match next.led(self, Some(node)) {
                Ok(n) => n,
                Err(_err) => {
                    return Err(ExpressionResult::LedError);
                }
            };
            next_token = match self.token_option.lookup_next_one() {
                Some(t) => t,
                None => {
                    return Ok(node);
                }
            }
        }
        Ok(node)
    }

    pub fn take_next_one(&mut self) -> Option<TokenBox> {
        self.token_option.take_next_one()
    }

    pub fn lookup_next_one(&self) -> Option<&TokenBox> {
        self.token_option.lookup_next_one()
    }

    pub fn new(token_option: TokenOption) -> Self {
        Self {
            token_option: token_option
        }
    }
}

