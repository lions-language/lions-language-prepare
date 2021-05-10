use crate::lexical;

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
        match lexical.lookup_next_one() {
            Some(n) => {
            },
            None => {
                panic!("expect int");
            }
        }
    }

    pub fn new() -> Self {
        Self {
        }
    }
}

