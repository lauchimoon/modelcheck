mod token;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;
use std::fs;
use std::env;
use std::path::Path;
use std::ffi::OsStr;

fn main() {
    let filepath = parse_args();
    let src = fs::read_to_string(filepath).expect("Error opening file");

    let mut lexer = Lexer::new(src);
    let tokens = lexer.lex();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();

    for stmt in stmts {
        println!("{} {} {:#?}", stmt.keyword, stmt.identifier, stmt.set);
    }
}

fn parse_args() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("{}: missing .model file", args[0]);
    }

    let filepath = Path::new(&args[1]);
    let ext = filepath.extension();
    if ext != Some(OsStr::new("model")) {
        panic!("{}: '{}' is not a .model file", args[0], filepath.display());
    }

    filepath.to_str().unwrap().to_string()
}
