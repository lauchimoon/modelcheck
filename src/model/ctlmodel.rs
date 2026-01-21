use std::fs;
use std::collections::HashMap;

use crate::model::lexer::Lexer;
use crate::model::parser::Parser;
use crate::model::interpreter::Interpreter;

#[derive(Default, Clone)]
pub struct CTLState {
    pub labels: Vec<String>,
    pub transitions: Vec<String>,
}

#[derive(Clone)]
pub struct Model {
    pub states: Vec<String>,
    pub init_states: Vec<String>,
    pub state_info: HashMap<String, CTLState>,
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
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        let stmts = parser.parse();

        // TODO: what happens if "let S" and "let I" are not present in the program?
        let mut interpreter = Interpreter::new(stmts);
        interpreter.interpret();

        interpreter.model
    }
}
