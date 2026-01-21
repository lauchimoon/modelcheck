mod token;
mod lexer;
mod parser;
mod interpreter;
mod model;

use model::Model;

use std::env;
use std::path::Path;
use std::ffi::OsStr;

fn main() {
    let filepath = parse_args();
    let model = Model::new(filepath);

    println!("S: {:#?}\nI: {:#?}", model.states, model.init_states);
    for (ident, state) in model.state_info {
        println!("{}: Labels: {:#?}\nTransitions: {:#?}", ident, state.labels, state.transitions);
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
