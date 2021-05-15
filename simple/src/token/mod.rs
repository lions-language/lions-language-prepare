pub struct TokenValue {
    pub value: String
}

#[derive(Debug)]
pub enum TokenType {
    Int,
    Identify,
    Number,
    Equal,
}

pub struct Token {
    pub typ: TokenType,
    pub value: TokenValue
}

