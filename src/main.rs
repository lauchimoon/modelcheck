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
}

fn main() {
    let _src = "let S {s0,s1,s2,s3}".to_string();

    let mut tokens: Vec<Token> = Vec::new();
    tokens.push(Token{kind: TokenKind::Keyword, value: "let".to_string()});
    tokens.push(Token{kind: TokenKind::Symbol, value: "S".to_string()});
    tokens.push(Token{kind: TokenKind::OpenCurly, value: "{".to_string()});
    tokens.push(Token{kind: TokenKind::Symbol, value: "s0".to_string()});
    tokens.push(Token{kind: TokenKind::Comma, value: ",".to_string()});
    tokens.push(Token{kind: TokenKind::Symbol, value: "s1".to_string()});
    tokens.push(Token{kind: TokenKind::Comma, value: ",".to_string()});
    tokens.push(Token{kind: TokenKind::Symbol, value: "s2".to_string()});
    tokens.push(Token{kind: TokenKind::Comma, value: ",".to_string()});
    tokens.push(Token{kind: TokenKind::Symbol, value: "s3".to_string()});
    tokens.push(Token{kind: TokenKind::CloseCurly, value: "}".to_string()});

    for tok in tokens {
        println!("{:#?}: {}", tok.kind, tok.value);
    }
}
