use crate::token;
use std::collections::VecDeque;

pub struct Lexical {
    content: VecDeque<u8>,
    tokens: VecDeque<token::Token>
}

impl Lexical {
    pub fn parse(&mut self) {
        loop {
            let next_ch = match self.lookup_next_one() {
                Some(v) => {
                    v
                },
                None => {
                    break;
                }
            };
            match next_ch {
                '=' => {
                    self.equal_process(next_ch);
                },
                '\r'|'\n'|'\t'|' ' => {
                    self.skip_next_one();
                },
                _ => {
                    self.other_char_parse(next_ch);
                }
            }
        }
    }

    fn equal_process(&mut self, c: char) {
        self.skip_next_one();
        let mut value = String::new();
        value.push(c);
        self.tokens.push_back(token::Token::Equal(value));
    }

    /*
     * 除第一位外, 字符是否属于ID
     * */
    fn is_id(&self, c: char) -> bool {
        if (c == '_') || (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') {
            return true;
        }
        false
    }

    fn is_id_start(&self, c: char) -> bool {
        if (c == '_') || (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') {
            return true;
        }
        false
    }

    /*
     * 是否是数字
     * */
    fn is_number_start(&self, c: char) -> bool {
        if c >= '0' && c <= '9' {
            return true;
        }
        false
    }

    fn id_process(&mut self, c: char) {
        let mut s = String::new();
        s.push(c);
        self.skip_next_one();
        loop {
            match self.lookup_next_one() {
                Some(c) => {
                    if self.is_id(c) {
                        s.push(c);
                        self.skip_next_one();
                    } else {
                        break;
                    }
                },
                None => {
                    break;
                }
            }
        }
        match s.as_str() {
            "var" => {
            },
            _ => {
            }
        }
    }

    fn number_is_10(&self, c: char) -> Option<u8> {
        if c >= '0' && c <= '9' {
            return Some(c as u8 - '0' as u8);
        }
        None
    }

    fn number_process(&mut self, start_c: char) {
        let mut value: String = String::new();
        loop {
            let c = match self.lookup_next_one() {
                Some(c) => c,
                None => {
                    break;
                }
            };
            if let Some(v) = self.number_is_10(c) {
                value.push(c);
                self.skip_next_one();
            } else {
                /*
                 * 非整数
                 * */
                self.tokens.push_back(token::Token::Number(value));
                break;
            }
        }
    }

    fn other_char_parse(&mut self, c: char) {
        if self.is_id_start(c) {
            self.id_process(c);
        } else if self.is_number_start(c) {
            self.number_process(c);
        } else {
            panic!("not support char: {}", c);
        }
    }

    pub fn lookup_next_n(&self, n: usize) -> Option<char> {
        if n > self.content.len() {
            return None;
        }
        match self.content.front() {
            Some(c) => Some(*c as char),
            None => None
        }
    }

    pub fn lookup_next_one(&self) -> Option<char> {
        self.lookup_next_n(1)
    }

    pub fn take_next_one(&mut self) -> Option<char> {
        match self.content.pop_front() {
            Some(c) => Some(c as char),
            None => None
        }
    }

    pub fn skip_next_one(&mut self) {
        if self.content.is_empty() {
            panic!("content is empty");
        }
        self.content.pop_front();
    }

    pub fn print_tokens(&self) {
        for token in self.tokens.iter() {
            println!("{:?}", token);
        }
    }

    pub fn tokens(self) -> VecDeque<token::Token> {
        self.tokens
    }

    pub fn new(content: Vec<u8>) -> Self {
        Self {
            content: VecDeque::from(content),
            tokens: VecDeque::new()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn lexical_test() {
        use std::fs;
        let content = fs::read_to_string("main.lions").unwrap();
        let content = content.as_bytes().to_vec();
        let mut lexical = Lexical::new(content);
        lexical.parse();
        lexical.print_tokens();
    }
}

