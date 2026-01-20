#[derive(Debug, PartialEq)]
pub enum Kind {
    Keyword,
    Symbol,
    OpenCurly,
    CloseCurly,
    Comma,
    Semicolon,
    Comment,
    Nil,
}

pub struct Token {
    pub kind: Kind,
    pub value: String,
    pub line: usize,
    pub loc: usize,
}
