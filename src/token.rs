#[derive(Debug)]
pub enum Kind {
    Keyword,
    Symbol,
    OpenCurly,
    CloseCurly,
    Comma,
}

pub struct Token {
    pub kind: Kind,
    pub value: String,
    pub line: usize,
}
