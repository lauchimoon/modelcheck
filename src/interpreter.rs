use std::collections::HashMap;
use crate::parser;

#[derive(Default)]
pub struct CTLState {
    pub labels: Vec<String>,
    pub transitions: Vec<String>,
}

type InitState = HashMap<String, Vec<String>>;
type Model = HashMap<String, CTLState>;

pub struct Interpreter {
    stmts: Vec<parser::Statement>,
    pub init: InitState,
    pub model: Model,
}

impl Interpreter {
    pub fn new(statements: Vec<parser::Statement>) -> Self {
        Interpreter {
            stmts: statements,
            init: HashMap::new(),
            model: HashMap::new(),
        }
    }

    pub fn interpret(&mut self) {
        for stmt in &self.stmts {
            if stmt.keyword == "let" {
                if !expect(stmt.identifier.clone(), &["S".to_string(), "I".to_string()]) {
                    panic!("eval error: expected 'S' or 'I' after let, got {}.", stmt.identifier);
                }
                self.init.entry(stmt.identifier.clone()).or_insert(stmt.set.clone());
            } else if stmt.keyword == "label" {
                let state = CTLState {
                    labels: stmt.set.clone(),
                    transitions: Vec::new(),
                };
                self.model.entry(stmt.identifier.clone()).or_insert(state);
            } else if stmt.keyword == "transition" {
                let state = self.model.entry(stmt.identifier.clone()).or_default();
                for st in &stmt.set {
                    state.transitions.push((*st).clone());
                }
            }
        }
    }

}

fn expect(target: String, candidates: &[String]) -> bool {
    for candidate in candidates {
        if *candidate == target {
            return true;
        }
    }
    false
}
