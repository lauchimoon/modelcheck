use crate::prop::token::Token;
use crate::prop::token::Kind;
use crate::prop::formula::Formula;

pub struct Parser {
    cursor: usize,
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            cursor: 0,
            tokens: tokens
        }
    }

    pub fn parse(&mut self) -> Formula {
        self.parse_implies()
    }

    fn parse_implies(&mut self) -> Formula {
        let mut l = self.parse_or();
        while self.current().kind == Kind::Implies {
            self.consume();
            let r = self.parse_implies();
            l = Formula::Implies(Box::new(l), Box::new(r));
        }

        l
    }

    fn parse_or(&mut self) -> Formula {
        let mut l = self.parse_and();
        while self.current().kind == Kind::Or {
            self.consume();
            let r = self.parse_and();
            l = Formula::Or(Box::new(l), Box::new(r));
        }

        l
    }

    fn parse_and(&mut self) -> Formula {
        let mut l = self.parse_unary();
        while self.current().kind == Kind::And {
            self.consume();
            let r = self.parse_unary();
            l = Formula::And(Box::new(l), Box::new(r));
        }

        l
    }

    fn parse_unary(&mut self) -> Formula {
        let token = self.current().clone();
        match token.kind {
            Kind::Variable => {
                self.consume();
                Formula::Var(token.value)
            }
            Kind::False => {
                self.consume();
                Formula::False
            }
            Kind::True => {
                self.consume();
                Formula::True
            }
            Kind::Not => {
                self.consume();
                let l = self.parse_unary();
                Formula::Not(Box::new(l))
            }
            Kind::OpenParen => {
                self.consume();
                let formula = self.parse();
                self.consume();
                let current = self.current().clone();
                if current.kind != Kind::CloseParen {
                    panic!("expected closing parenthesis, found {:#?}", current.kind);
                }
                self.consume();
                formula
            }
            Kind::Exists => {
                self.consume();
                let modifier = self.current().clone();
                self.parse_exists_op(modifier.kind)
            }
            Kind::ForAll => {
                self.consume();
                let modifier = self.current().clone();
                self.parse_forall_op(modifier.kind)
            }
            _ => todo!()
        }
    }

    fn parse_exists_op(&mut self, modifier: Kind) -> Formula {
        match modifier {
            Kind::Next => {
                self.consume();
                let formula = self.parse();
                Formula::EX(Box::new(formula))
            }
            Kind::OpenBracket => {
                self.consume();
                let l = self.parse();
                let mut current = self.current().clone();
                // Only operator to use [] is until
                if current.kind != Kind::Until {
                    panic!("expected U, found {:#?}", current.kind);
                }
                self.consume();
                let r = self.parse();
                current = self.current().clone();
                if current.kind != Kind::CloseBracket {
                    panic!("expected closing bracket, found {:#?}", current.kind);
                }
                Formula::EU(Box::new(l), Box::new(r))
            }
            Kind::Future => {
                self.consume();
                let formula = self.parse();
                Formula::EF(Box::new(formula))
            }
            Kind::Global => {
                self.consume();
                let formula = self.parse();
                Formula::EG(Box::new(formula))
            }
            _ => todo!()
        }
    }

    fn parse_forall_op(&mut self, modifier: Kind) -> Formula {
        match modifier {
            Kind::Next => {
                self.consume();
                let formula = self.parse();
                Formula::AX(Box::new(formula))
            }
            Kind::OpenBracket => {
                self.consume();
                let l = self.parse();
                let mut current = self.current().clone();
                // Only operator to use [] is until
                if current.kind != Kind::Until {
                    panic!("expected U, found {:#?}", current.kind);
                }
                self.consume();
                let r = self.parse();
                current = self.current().clone();
                if current.kind != Kind::CloseBracket {
                    panic!("expected closing bracket, found {:#?}", current.kind);
                }
                Formula::AU(Box::new(l), Box::new(r))
            }
            Kind::Future => {
                self.consume();
                let formula = self.parse();
                Formula::AF(Box::new(formula))
            }
            Kind::Global => {
                self.consume();
                let formula = self.parse();
                Formula::AG(Box::new(formula))
            }
            _ => todo!()
        }
    }

    fn consume(&mut self) {
        if self.cursor + 1 >= self.tokens.len() {
            return;
        }
        self.cursor += 1
    }

    fn current(&mut self) -> &Token {
        if self.cursor >= self.tokens.len() {
            return self.previous();
        }
        &self.tokens[self.cursor]
    }

    fn previous(&mut self) -> &Token {
        let mut prev_idx = self.cursor - 1;
        if self.cursor == 0 {
            prev_idx = 0;
        }
        &self.tokens[prev_idx]
    }
}
