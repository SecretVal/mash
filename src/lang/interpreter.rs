#![allow(dead_code)]
use std::io::{self, Stdout, Write};

use super::parser::*;

#[derive(Debug)]
pub struct Intepreter{
    input: Vec<Statement>,
    output: String,
    pos: usize,
    stdout: Stdout,
}

impl Intepreter {
    pub fn new(input: Vec<Statement>) -> Self {
	Self {
	    input,
	    output: String::new(),
	    pos: 0,
	    stdout: io::stdout(),
	}
    }

    pub fn execute(&mut self) -> Result<(), &'static str>  {
	let s = &self.input[self.pos];
	match &s.kind {
	    StatementKind::Expression(expression) => {
		match expression.kind {
		    ExpressionKind::NumberExpression(num) => self.print_number(num),
		    ExpressionKind::PlusExpression(n1, n2) => self.print_number(n1 + n2),
		    ExpressionKind::MinusExpression(n1, n2) => self.print_number(n1 - n2),
		}
	    },
	}
	    
	Ok(())
    }

    fn print_number(&mut self, n: u128) {
	self.stdout.write(format!("{n}\n").as_bytes()).unwrap();
    }
}
