pub struct TokenValue {
    pub value: String
}

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

