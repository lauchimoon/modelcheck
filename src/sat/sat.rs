use std::collections::HashSet;
use crate::prop::formula::Formula;
use crate::model::ctlmodel::Model;

// Sat(False)      = empty
// Sat(p)          = {s \in model.states s.t. p \in model.state_info[s].labels)}
// Sat(~phi)       = model.states - Sat(phi)
// Sat(phi ^ psi)  = Sat(phi) \cap Sat(psi)
// Sat(phi v psi)  = Sat(phi) \cup Sat(psi)
// Sat(phi -> psi) = Sat(~phi) \cup Sat(psi)
// Sat(EXphi)      = pre_exists(Sat(phi))
// Sat(AXphi)      = pre_forall(Sat(phi))
pub fn sat(model: &Model, formula: &Formula) -> HashSet<String> {
    match formula {
        Formula::False => {
            HashSet::new()
        }
        Formula::Var(ident) => {
            let mut states: HashSet<String> = HashSet::new();
            for state in &model.states {
                for label in &model.state_info[state].labels {
                    if label == ident {
                        states.insert(state.to_string());
                    }
                }
            }
            states
        }
        Formula::Not(form) => {
            let s: HashSet<String> = model.states.clone().into_iter().collect();
            s.difference(&sat(model, form)).cloned().collect()
        }
        Formula::And(form1, form2) => {
            let sat1 = sat(model, form1);
            let sat2 = sat(model, form2);
            sat1.intersection(&sat2).cloned().collect()
        }
        Formula::Or(form1, form2) => {
            let sat1 = sat(model, form1);
            let sat2 = sat(model, form2);
            sat1.union(&sat2).cloned().collect()
        }
        Formula::Implies(form1, form2) => {
            // p -> q = ~p v q
            let equiv = Formula::Or(Box::new(Formula::Not(form1.clone())), form2.clone());
            sat(model, &equiv)
        }
        Formula::EX(form) => {
            pre_exists(model, sat(model, form))
        }
        Formula::AX(form) => {
            pre_forall(model, sat(model, form))
        }
        _ => todo!()
    }
}

// pre_exists(Y) = {s \in model.states s.t. there exists s' s.t. s' \in
// model.state_info[s].transitions && s' \in Y}
fn pre_exists(model: &Model, set: HashSet<String>) -> HashSet<String> {
    let mut pre: HashSet<String> = HashSet::new();
    for state in &model.states {
        let transitions = &model.state_info[state].transitions;
        for statep in transitions {
            if set.contains(statep) {
                pre.insert(state.to_string());
                break;
            }
        }
    }
    pre
}

// pre_forall(Y) = {s \in model.states s.t. for all s' s.t. s' \in
// model.state_info[s].transitions, s' \in Y}
fn pre_forall(model: &Model, set: HashSet<String>) -> HashSet<String> {
    let mut pre: HashSet<String> = HashSet::new();
    for state in &model.states {
        let mut belongs = true;
        let transitions = &model.state_info[state].transitions;
        for statep in transitions {
            if !set.contains(statep) {
                belongs = false;
                break;
            }
        }
        if belongs {
            pre.insert(state.to_string());
        }
    }
    pre
}
