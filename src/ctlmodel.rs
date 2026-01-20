use crate::interpreter::InitState;
use crate::interpreter::States;
use std::fs;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interpreter;

pub fn new(filepath: String) -> (InitState, States) {
    let src = fs::read_to_string(filepath).expect("Error opening file");

    let mut lexer = Lexer::new(src);
    let tokens = lexer.lex();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();

    // TODO: what happens if "let S" and "let I" are not present in the program?
    let mut interpreter = Interpreter::new(stmts);
    interpreter.interpret();

    let (init, states) = (interpreter.init, interpreter.states);
    (init, states)
}
