use crate::token::Token;
use crate::token::Kind;

pub struct Lexer {
    src: Vec<char>,
    src_len: usize,
    pub cursor: usize,
    pub line: usize,
}

impl Lexer {
    pub fn new(src: String) -> Self {
        return Lexer{
            src: src.chars().collect(),
            src_len: src.len(),
            cursor: 0,
            line: 1,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while self.cursor < self.src_len {
            let mut c = self.chop();
            if c.is_alphabetic() {
                let mut tok = String::new();
                tok.push(c);
                c = self.chop();
                while c.is_alphabetic() || c.is_digit(10) {
                    tok.push(c);
                    c = self.chop();
                }

                let kind: Kind;
                if is_keyword(&tok) {
                    kind = Kind::Keyword;
                } else if tok == "nil" {
                    kind = Kind::Nil;
                } else {
                    kind = Kind::Symbol;
                }

                tokens.push(Token {
                    kind: kind,
                    value: tok,
                    line: self.line,
                });
            }

            if c == '#' {
                tokens.push(Token {
                    kind: Kind::Comment,
                    value: "#".to_string(),
                    line: self.line,
                });

                c = self.chop();
                let mut comment_text = String::new();
                while c != '\n' {
                    comment_text.push(c);
                    c = self.chop();
                }

                tokens.push(Token {
                    kind: Kind::Comment,
                    value: comment_text,
                    line: self.line,
                });
            }

            if c == '{' {
                tokens.push(Token {
                    kind: Kind::OpenCurly,
                    value: "{".to_string(),
                    line: self.line,
                });
            } else if c == '}' {
                tokens.push(Token {
                    kind: Kind::CloseCurly,
                    value: "}".to_string(),
                    line: self.line,
                });
            } else if c == ',' {
                tokens.push(Token {
                    kind: Kind::Comma,
                    value: ",".to_string(),
                    line: self.line,
                });
            } else if c == '\n' {
                self.line += 1;
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
}

fn is_keyword(s: &String) -> bool {
    s == "let" || s == "label" || s == "transition"
}
