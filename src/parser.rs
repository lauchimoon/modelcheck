use crate::token::Token;
use crate::token::Kind;

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

pub struct Statement {
    pub keyword: String,
    pub identifier: String,
    pub set: Vec<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            cursor: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut stmts: Vec<Statement> = Vec::new();
        while self.cursor < self.tokens.len() {
            stmts.push(self.statement());
        }

        stmts
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

    // <statement> ::= <keyword> <symbol> <set>;
    fn statement(&mut self) -> Statement {
        if !self.matches(&[Kind::Keyword]) {
            syntax_error("keyword", self.current());
        }
        let kw = self.previous().value.clone();

        if !self.matches(&[Kind::Symbol]) {
            syntax_error("symbol", self.current());
        }
        let ident = self.previous().value.clone();

        let elements = self.set();

        if !self.matches(&[Kind::Semicolon]) {
            syntax_error("';'", self.current());
        }
        Statement {keyword: kw, identifier: ident, set: elements}
    }

    // <set> ::= {<element>} | nil
    fn set(&mut self) -> Vec<String> {
        let mut elements: Vec<String> = Vec::new();
        if !self.matches(&[Kind::Nil, Kind::OpenCurly]) {
            syntax_error("nil or '{'", self.current());
        }

        if self.previous().kind != Kind::Nil {
            self.element(&mut elements);

            // After parsing elements, check if '}' is present
            if !self.matches(&[Kind::CloseCurly]) {
                syntax_error("'}'", self.current());
            }
        }

        elements
    }

    // <element> ::= <symbol> | ,<element>
    fn element(&mut self, elems_vec: &mut Vec<String>) {
        if !self.matches(&[Kind::Symbol]) {
            syntax_error("Symbol", self.current());
        }

        (*elems_vec).push(self.previous().value.clone());

        if self.current().kind != Kind::CloseCurly {
            if !self.matches(&[Kind::Comma]) {
                syntax_error("','", self.current());
            }

            self.element(elems_vec);
        }
    }
}

fn syntax_error(expected: &str, tok: &Token) {
    panic!("Syntax error in {}:{}: expected {} but got {:#?} with value '{}'",
                tok.line, tok.loc, expected, tok.kind, tok.value);
}
