use crate::ctl::lexer::Lexer;
use crate::ctl::parser::Parser;
use crate::ctl::interpreter::Interpreter;
use crate::ctl::state::State;
use crate::sat::sat::sat;

use crate::prop::lexer::Lexer as PropLexer;
use crate::prop::parser::Parser as PropParser;
use crate::prop::formula::Formula;

use crate::util::set::print_set;

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

    pub fn new(filepath: String) -> Self {
        let src = fs::read_to_string(filepath).expect("Error opening file");

        let mut lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer.lex());
        let stmts = parser.parse();

        // TODO: what happens if "let S" and "let I" are not present in the program?
        let mut interpreter = Interpreter::new(stmts);
        interpreter.interpret();
        interpreter.model
    }

    pub fn check(&self, formula: &String) -> bool {
        let form = self.parse_formula(formula);
        let states = sat(self, &form);
        print!("Sat({formula}) = ");
        print_set(&states);

        let initial: HashSet<String> = self.init_states.clone().into_iter().collect();
        initial.is_subset(&states)
    }

    fn parse_formula(&self, formula_string: &String) -> Formula {
        PropParser::new(PropLexer::new(formula_string.clone()).lex()).parse()
    }
}
