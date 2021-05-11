use crate::lexical;
use crate::token;
use std::collections;

struct VarDeclare {
}

impl VarDeclare {
    /*
     * 语句: int a = 1
     * 文法: Intdeclare: 'int' Identify '=' Number
     * */
    pub fn parse1() {
        let content = r"
        int a = 1
        ";
        let mut lexical = lexical::Lexical::new(content.as_bytes().to_vec());
        lexical.parse();
        let tokens = lexical.tokens();
        let token_option = token::TokenOption::new(tokens);
    }

    pub fn new() -> Self {
        Self {
        }
    }
}

