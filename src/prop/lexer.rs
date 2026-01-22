use crate::prop::token::Token;
use crate::prop::token::Kind;

pub struct Lexer {
    src: Vec<char>,
    src_len: usize,
    cursor: usize,
}

impl Lexer {
    pub fn new(src: String) -> Self {
        Lexer {
            src: src.chars().collect(),
            src_len: src.len(),
            cursor: 0,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while self.cursor < self.src_len {
            let mut c = self.chop();

            if c == '0' {
                tokens.push(Token {
                    kind: Kind::False,
                    value: "f".to_string(),
                });
            } else if c == '1' {
                tokens.push(Token {
                    kind: Kind::True,
                    value: "t".to_string(),
                })
            } else if c == 'A' {
                tokens.push(Token {
                    kind: Kind::ForAll,
                    value: "A".to_string(),
                });
            } else if c == 'E' {
                tokens.push(Token {
                    kind: Kind::Exists,
                    value: "E".to_string(),
                });
            } else if c == 'X' {
                tokens.push(Token {
                    kind: Kind::Next,
                    value: "X".to_string(),
                });
            } else if c == 'U' {
                tokens.push(Token {
                    kind: Kind::Until,
                    value: "U".to_string(),
                });
            } else if c == 'F' {
                tokens.push(Token {
                    kind: Kind::Future,
                    value: "F".to_string(),
                });
            } else if c == 'G' {
                tokens.push(Token {
                    kind: Kind::Global,
                    value: "G".to_string(),
                });
            } else if c.is_alphabetic() {
                let mut val = String::new();
                val.push(c);
                c = self.chop();
                while c.is_alphabetic() || c.is_digit(10) {
                    val.push(c);
                    c = self.chop();
                }

                tokens.push(Token {
                    kind: Kind::Variable,
                    value: val,
                });
            }

            if c == '~' {
                tokens.push(Token {
                    kind: Kind::Not,
                    value: "~".to_string(),
                });
            } else if c == '^' {
                tokens.push(Token {
                    kind: Kind::And,
                    value: "and".to_string(),
                });
            } else if c == 'V' {
                tokens.push(Token {
                    kind: Kind::Or,
                    value: "or".to_string(),
                });
            } else if c == '-' && self.peek() == '>' {
                tokens.push(Token {
                    kind: Kind::Implies,
                    value: "implies".to_string(),
                });
            } else if c == '[' {
                tokens.push(Token {
                    kind: Kind::OpenBracket,
                    value: "[".to_string(),
                });
            } else if c == ']' {
                tokens.push(Token {
                    kind: Kind::CloseBracket,
                    value: "]".to_string(),
                });
            } else if c == '(' {
                tokens.push(Token {
                    kind: Kind::OpenParen,
                    value: "(".to_string(),
                });
            } else if c == ')' {
                tokens.push(Token {
                    kind: Kind::CloseParen,
                    value: ")".to_string(),
                });
            }
        }
        tokens
    }

    fn chop(&mut self) -> char {
        if self.cursor >= self.src_len {
            return ' ';
        }
        let c = self.src[self.cursor];
        self.cursor += 1;
        c
    }

    fn peek(&mut self) -> char {
        if self.cursor >= self.src_len {
            return ' ';
        }
        self.src[self.cursor]
    }
}
