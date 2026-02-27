mod model;
mod prop;
mod sat;

use crate::model::ctlmodel::Model;
use crate::prop::lexer::Lexer;
use crate::prop::parser::Parser;
use crate::prop::formula::Formula;
use crate::sat::sat::sat;

use std::env;
use std::path::Path;
use std::ffi::OsStr;

fn main() {
    let model = load_model();
    println!("S: {:#?}\nI: {:#?}", model.states, model.init_states);
    for (ident, state) in &model.state_info {
        println!("{}: Labels: {:#?}\nTransitions: {:#?}", ident, state.labels, state.transitions);
    }

    //let formula = parse_formula("E[~c U (b ^ ~t)]".to_string());
    let formula = parse_formula("AXb".to_string());
    for state in sat(&model, &formula) {
        println!("{}", state);
    }
}

fn load_model() -> Model {
    let filepath = parse_args();
    Model::new(filepath)
}

fn parse_formula(formula_string: String) -> Formula {
    Parser::new(Lexer::new(formula_string).lex()).parse()
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
