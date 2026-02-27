mod model;
mod prop;
mod sat;

use crate::model::ctlmodel::Model;
use std::env;
use std::path::Path;
use std::ffi::OsStr;

fn main() {
    let model = load_model();
    println!("S: {:#?}\nI: {:#?}", model.states, model.init_states);
    for (ident, state) in &model.state_info {
        println!("{}: Labels: {:#?}\nTransitions: {:#?}", ident, state.labels, state.transitions);
    }

    let valid = model.check("E[~c U (b ^ ~t)]".to_string());
    println!("{}", valid);
}

fn load_model() -> Model {
    let filepath = parse_args();
    Model::new(filepath)
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
