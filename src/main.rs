mod model;
mod prop;

use crate::model::ctlmodel::Model;
use crate::prop::lexer::Lexer;

use std::env;
use std::path::Path;
use std::ffi::OsStr;

fn main() {
    let model = load_model();
    println!("S: {:#?}\nI: {:#?}", model.states, model.init_states);
    for (ident, state) in model.state_info {
        println!("{}: Labels: {:#?}\nTransitions: {:#?}", ident, state.labels, state.transitions);
    }

    run_prop("E[~c U (b ^ ~t)]".to_string());
    run_prop("p -> AGp".to_string());
    run_prop("A[0 U p] -> p".to_string());
    run_prop("EGp -> AFp".to_string());
}

fn load_model() -> Model {
    let filepath = parse_args();
    Model::new(filepath)
}

fn run_prop(prop: String) {
    println!("{}", prop);
    let mut lexer = Lexer::new(prop);
    let tokens = lexer.lex();
    for tok in &tokens {
        println!("{:#?}: {}", tok.kind, tok.value);
    }
    println!("-------");
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
