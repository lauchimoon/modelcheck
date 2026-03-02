use crate::prop::lexer::Lexer;
use crate::prop::parser::Parser;

use std::fmt;

#[derive(Clone, Debug)]
pub enum Formula {
    False,
    True,
    Var(String),
    Not(Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Implies(Box<Formula>, Box<Formula>),
    EX(Box<Formula>),               // Exists next p
    AX(Box<Formula>),               // For all next p
    EU(Box<Formula>, Box<Formula>), // E[p1 U p2]
    AU(Box<Formula>, Box<Formula>), // A[p1 U p2]
    EF(Box<Formula>),
    AF(Box<Formula>),
    EG(Box<Formula>),
    AG(Box<Formula>)
}

impl Formula {
    pub fn from_string(string: String) -> Self {
        Parser::new(Lexer::new(string).lex()).parse()
    }
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Formula::False => write!(f, "0"),
            Formula::True => write!(f, "1"),
            Formula::Var(ident) => write!(f, "{ident}"),
            Formula::Not(formula) => write!(f, "(~{formula})"),
            Formula::And(form1, form2) => write!(f, "({form1} ^ {form2})"),
            Formula::Or(form1, form2) => write!(f, "({form1} v {form2})"),
            Formula::Implies(form1, form2) => write!(f, "({form1} -> {form2})"),
            Formula::EX(formula) => write!(f, "EX{formula}"),
            Formula::AX(formula) => write!(f, "AX{formula}"),
            Formula::EU(form1, form2) => write!(f, "E[{form1} U {form2}]"),
            Formula::AU(form1, form2) => write!(f, "A[{form1} U {form2}]"),
            Formula::EF(formula) => write!(f, "EF{formula}"),
            Formula::AF(formula) => write!(f, "AF{formula}"),
            Formula::EG(formula) => write!(f, "EG{formula}"),
            Formula::AG(formula) => write!(f, "AG{formula}"),
        }
    }
}
