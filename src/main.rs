mod token;
mod lexer;
use lexer::Lexer;
use std::fs;
use std::env;
use std::path::Path;
use std::ffi::OsStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("{}: missing .model file", args[0]);
    }

    let filepath = Path::new(&args[1]);
    let ext = filepath.extension();
    if ext != Some(OsStr::new("model")) {
        panic!("{}: '{}' is not a .model file", args[0], filepath.display());
    }

    let src = fs::read_to_string(filepath).expect("Error opening file");
    let mut lexer = Lexer::new(src);
    let tokens = lexer.lex();
    for tok in tokens {
        println!("{:#?}: {} @ line {}", tok.kind, tok.value, tok.line);
    }
}
