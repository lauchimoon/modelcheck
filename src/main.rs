mod ctl;
mod prop;
mod sat;
mod util;

use crate::ctl::model::Model;
use crate::util::set::print_set;

use std::env;
use std::path::Path;
use std::ffi::OsStr;

fn main() {
    let (model, prop) = load_model_and_prop();
    print!("S = ");
    print_set(&model.states);
    print!("I = ");
    print_set(&model.init_states);
    for (ident, state) in &model.state_info {
        println!("{}:", ident);
        print!("  Labels: ");
        print_set(&state.labels);
        print!("  Transitions: ");
        print_set(&state.transitions);
    }
    let valid = model.check(&prop);
    if valid {
        println!("M |= {}", prop);
    } else {
        println!("M |/= {}", prop);
    }
}

fn load_model_and_prop() -> (Model, String) {
    let (filepath, prop) = parse_args();
    (Model::new(filepath), prop)
}

fn parse_args() -> (String, String) {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("{}: missing .model file and proposition", args[0]);
    }
    let filepath = Path::new(&args[1]);
    let ext = filepath.extension();
    if ext != Some(OsStr::new("model")) {
        panic!("{}: '{}' is not a .model file", args[0], filepath.display());
    }
    (filepath.to_str().unwrap().to_string(), args[2].clone())
}
