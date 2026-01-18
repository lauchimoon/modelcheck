#[derive(Debug)]
enum TokenKind {
    Keyword,
    Symbol,
    OpenCurly,
    CloseCurly,
    Comma,
}

struct Token {
    kind: TokenKind,
    value: String,
    line: usize,
}

struct Lexer {
    src: Vec<char>,
    src_len: usize,
    cursor: usize,
    line: usize,
}

fn main() {
    let src = "let S {s0,s1,s2,s3}".to_string();
    let mut lexer = Lexer::new(src);
    let tokens = lexer.lex();

    for tok in tokens {
        println!("{:#?}: {} @ line {}", tok.kind, tok.value, tok.line);
    }
}

impl Lexer {
    fn new(src: String) -> Self {
        return Lexer{
            src: src.chars().collect(),
            src_len: src.len(),
            cursor: 0,
            line: 1,
        }
    }

    fn chop(&mut self) -> char {
        if self.cursor >= self.src_len {
            return ' ';
        }
        let c = self.src[self.cursor];
        self.cursor += 1;
        c
    }

    fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while self.cursor < self.src_len {
            let c = self.chop();
            if c == '{' {
                tokens.push(Token {
                    kind: TokenKind::OpenCurly,
                    value: "{".to_string(),
                    line: self.line,
                });
            } else if c == '}' {
                tokens.push(Token {
                    kind: TokenKind::CloseCurly,
                    value: "}".to_string(),
                    line: self.line,
                });
            } else if c == ',' {
                tokens.push(Token {
                    kind: TokenKind::Comma,
                    value: ",".to_string(),
                    line: self.line,
                });
            } else if c == '\n' {
                self.line += 1;
            }
        }
        tokens
    }
}
