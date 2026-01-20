mod token;
mod lexer;
mod parser;

use std::collections::HashMap;
use lexer::Lexer;
use parser::Parser;
use std::fs;
use std::env;
use std::path::Path;
use std::ffi::OsStr;

#[derive(Debug, Default)]
struct CTLState {
    labels: Vec<String>,
    transitions: Vec<String>,
}

type InitState = HashMap<String, Vec<String>>;
type Model = HashMap<String, CTLState>;

fn main() {
    let filepath = parse_args();
    let src = fs::read_to_string(filepath).expect("Error opening file");

    let mut lexer = Lexer::new(src);
    let tokens = lexer.lex();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();

    let mut init: InitState = HashMap::new();
    let mut model: Model = HashMap::new();

    // TODO: what happens if "let S" and "let I" are not present in the program?
    eval(&stmts, &mut init, &mut model);
    println!("S: {:#?}\nI: {:#?}", init["S"], init["I"]);

    for (ident, st) in model {
        println!("{}: Labels: {:#?}\nTransitions: {:#?}", ident, st.labels, st.transitions);
    }
}

fn eval(stmts: &Vec<parser::Statement>, init: &mut InitState, model: &mut Model) {
    for stmt in stmts {
        if stmt.keyword == "let" {
            if !expect(stmt.identifier.clone(), &["S".to_string(), "I".to_string()]) {
                panic!("eval error: expected 'S' or 'I' after let, got {}.", stmt.identifier);
            }
            init.entry(stmt.identifier.clone()).or_insert(stmt.set.clone());
        } else if stmt.keyword == "label" {
            let state = CTLState {
                labels: stmt.set.clone(),
                transitions: Vec::new(),
            };
            model.entry(stmt.identifier.clone()).or_insert(state);
        } else if stmt.keyword == "transition" {
            let state = model.entry(stmt.identifier.clone()).or_default();
            for st in &stmt.set {
                state.transitions.push((*st).clone());
            }
        }
    }
}

fn expect(target: String, candidates: &[String]) -> bool {
    for candidate in candidates {
        if *candidate == target {
            return true;
        }
    }
    false
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
