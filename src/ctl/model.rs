use crate::ctl::lexer::Lexer;
use crate::ctl::parser::Parser;
use crate::ctl::interpreter::Interpreter;
use crate::ctl::state::State;
use crate::prop::formula::Formula;
use crate::util::set::print_set;
use crate::sat::sat::sat;

use std::fmt;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Model {
    pub states: Vec<String>,
    pub init_states: Vec<String>,
    pub state_info: HashMap<String, State>,
}

impl Model {
    pub fn empty() -> Self {
        Model {
            states: vec![],
            init_states: vec![],
            state_info: HashMap::new(),
        }
    }

    pub fn from_file(filepath: String) -> Self {
        let src = fs::read_to_string(filepath).expect("Error opening file");

        let mut lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer.lex());
        let stmts = parser.parse();

        // TODO: what happens if "let S" and "let I" are not present in the program?
        let mut interpreter = Interpreter::new(stmts);
        interpreter.interpret();
        interpreter.model
    }

    pub fn check(&self, formula: &Formula) -> bool {
        let states = sat(self, formula);
        let mut buffer = String::new();
        let _ = print_set(&mut buffer, &states);
        println!("Sat({formula}) = {}", buffer);

        let initial: HashSet<String> = self.init_states.clone().into_iter().collect();
        initial.is_subset(&states)
    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "S = ")?;
        let _ = print_set(f, &self.states);
        write!(f, "\nI = ")?;
        let _ = print_set(f, &self.init_states);
        for (ident, state) in &self.state_info {
            writeln!(f, "\n{}:", ident)?;
            write!(f, "  Labels: ")?;
            let _ = print_set(f, &state.labels);
            write!(f, "\n  Transitions: ")?;
            let _ = print_set(f, &state.transitions);
        }
        Ok(())
    }
}
