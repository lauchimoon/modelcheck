use std::collections::HashSet;
use crate::prop::formula::Formula;
use crate::model::ctlmodel::Model;

// Sat(False)      = empty
// Sat(p)          = {s \in model.states s.t. p \in model.state_info[s].labels)}
// Sat(~phi)       = model.states - Sat(phi)
// Sat(phi ^ psi)  = Sat(phi) \cap Sat(psi)
// Sat(phi v psi)  = Sat(phi) \cup Sat(psi)
// Sat(phi -> psi) = Sat(~phi) \cup Sat(psi)
pub fn Sat(model: &Model, formula: &Formula) -> HashSet<String> {
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
            let S: HashSet<String> = model.states.clone().into_iter().collect();
            S.difference(&Sat(model, form)).cloned().collect()
        }
        Formula::And(form1, form2) => {
            let sat1 = Sat(model, form1);
            let sat2 = Sat(model, form2);
            sat1.intersection(&sat2).cloned().collect()
        }
        Formula::Or(form1, form2) => {
            let sat1 = Sat(model, form1);
            let sat2 = Sat(model, form2);
            sat1.union(&sat2).cloned().collect()
        }
        Formula::Implies(form1, form2) => {
            // p -> q = ~p v q
            let equiv = Formula::Or(Box::new(Formula::Not(form1.clone())), form2.clone());
            Sat(model, &equiv)
        }
        _ => todo!()
    }
}
