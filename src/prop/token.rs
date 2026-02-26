#[derive(Debug, PartialEq, Clone)]
pub enum Kind {
    False,
    True,
    Variable,
    Not,
    And,
    Or,
    Implies,
    ForAll,
    Exists,
    Next,
    Until,
    Future,
    Global,
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: Kind,
    pub value: String,
}
