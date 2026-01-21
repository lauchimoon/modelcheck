use crate::parser;
use crate::model::Model;
use crate::model::CTLState;

pub struct Interpreter {
    stmts: Vec<parser::Statement>,
    pub model: Model,
}

impl Interpreter {
    pub fn new(statements: Vec<parser::Statement>) -> Self {
        Interpreter {
            stmts: statements,
            model: Model::empty(),
        }
    }

    pub fn interpret(&mut self) -> Model {
        for stmt in &self.stmts {
            if stmt.keyword == "let" {
                if !expect(stmt.identifier.clone(), &["S".to_string(), "I".to_string()]) {
                    panic!("eval error: expected 'S' or 'I' after let, got {}.", stmt.identifier);
                }

                if stmt.identifier == "S" {
                    self.model.states = stmt.set.clone();
                } else if stmt.identifier == "I" {
                    self.model.init_states = stmt.set.clone();
                }
            } else if stmt.keyword == "label" {
                let state = CTLState {
                    labels: stmt.set.clone(),
                    transitions: Vec::new(),
                };
                self.model.state_info.entry(stmt.identifier.clone()).or_insert(state);
            } else if stmt.keyword == "transition" {
                let state = self.model.state_info.entry(stmt.identifier.clone()).or_default();
                for st in &stmt.set {
                    state.transitions.push((*st).clone());
                }
            }
        }

        self.model.clone()
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
