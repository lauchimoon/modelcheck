#[derive(Debug)]
pub enum Formula {
    False,
    True,
    Var(String),
    Not(Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Implies(Box<Formula>, Box<Formula>),
    EX(Box<Formula>),               // Eo(p)
    AX(Box<Formula>),               // Ao(p)
    EU(Box<Formula>, Box<Formula>), // E[p1 U p2]
    AU(Box<Formula>, Box<Formula>), // A[p1 U p2]
    EF(Box<Formula>),
    AF(Box<Formula>),
    EG(Box<Formula>),
    AG(Box<Formula>)
}
