mod ctl;
mod prop;
mod sat;
mod util;

use crate::ctl::model::Model;
use crate::prop::formula::Formula;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Path to a model file
    model_path: String,

    /// Formula to test the model on
    formula: String,

    /// Display information about the model
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    let model = Model::from_file(args.model_path);
    let formula = Formula::from_string(args.formula);

    if args.verbose {
        println!("{}", model);
    }

    let valid = model.check(&formula);
    if valid {
        println!("M |= {}", formula);
    } else {
        println!("M |/= {}", formula);
    }
}
