use crate::token::Token;
use crate::token::Kind;

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            cursor: 0,
        }
    }

    pub fn parse(&mut self) {
        self.statement();
    }

    fn previous(&self) -> &Token {
        let mut prev_idx = self.cursor - 1;
        if self.cursor == 0 {
            prev_idx = self.cursor;
        }

        &self.tokens[prev_idx]
    }

    fn current(&self) -> &Token {
        if self.cursor >= self.tokens.len() {
            return self.previous();
        }
        &self.tokens[self.cursor]
    }

    fn matches(&mut self, kinds: &[Kind]) -> bool {
        for kind in kinds {
            if self.current().kind == *kind {
                self.cursor += 1;
                return true;
            }
        }

        false
    }

    // <statement> ::= <keyword> <symbol> <set>
    fn statement(&mut self) {
        println!("<statement>");
        if !self.matches(&[Kind::Keyword]) {
            syntax_error("keyword", self.current());
        }
        println!("<keyword>{}</keyword>", self.previous().value);

        if !self.matches(&[Kind::Symbol]) {
            syntax_error("symbol", self.current());
        }
        println!("<symbol>{}</symbol>", self.previous().value);

        self.set();
        println!("</statement>");
    }

    // <set> ::= {<element>} | nil
    fn set(&mut self) {
        if !self.matches(&[Kind::Nil, Kind::OpenCurly]) {
            syntax_error("nil or '{'", self.current());
        }
        println!("<set>");

        if self.current().kind != Kind::Nil {
            self.element();
        }

        println!("</set>");
    }

    // <element> ::= <symbol> | ,<element>
    fn element(&mut self) {
        if !self.matches(&[Kind::Symbol]) {
            syntax_error("Symbol", self.current());
        }
        println!("<element>{}</element>", self.previous().value);

        if self.cursor >= self.tokens.len() {
            syntax_error("'}'", self.current());
        }

        if self.current().kind != Kind::CloseCurly {
            if !self.matches(&[Kind::Comma]) {
                syntax_error("','", self.current());
            }

            self.element();
        }
    }
}

fn syntax_error(expected: &str, tok: &Token) {
    panic!("Syntax error in {}:{}: expected {} but got {:#?} with value '{}'",
                tok.line, tok.loc, expected, tok.kind, tok.value);
}
