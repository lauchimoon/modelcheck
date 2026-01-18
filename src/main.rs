mod token;
mod lexer;
use lexer::Lexer;

fn main() {
    let src = "let S {s0,s1,s2,s3}".to_string();
    let mut lexer = Lexer::new(src);
    let tokens = lexer.lex();

    for tok in tokens {
        println!("{:#?}: {} @ line {}", tok.kind, tok.value, tok.line);
    }
}
