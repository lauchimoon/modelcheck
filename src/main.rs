mod ctl;
mod prop;
mod sat;
mod util;

use crate::ctl::model::Model;
use crate::prop::formula::Formula;

use std::env;
use std::path::Path;
use std::ffi::OsStr;

fn main() {
    let (model_path, formula_string) = parse_args();
    let model = Model::from_file(model_path);
    let formula = Formula::from_string(formula_string);

    let verbose = true;
    if verbose {
        println!("{}", model);
    }

    let valid = model.check(&formula);
    if valid {
        println!("M |= {}", formula);
    } else {
        println!("M |/= {}", formula);
    }
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
