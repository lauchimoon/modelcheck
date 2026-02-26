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
                let current = self.current().clone();
                if current.kind != Kind::CloseParen {
                    panic!("expected closing parenthesis, found {:#?}", current.kind);
                }
                self.consume();
                formula
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
