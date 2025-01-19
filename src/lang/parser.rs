#![allow(dead_code)]
use std::{thread::current, u128};

use super::lexer::*;

#[derive(Debug, Default)]
pub struct Parser {
    input: Vec<Token>,
    output: Vec<Statement>,
    pos: usize,
}

#[derive(Debug)]
pub struct Statement {
    kind: StatementKind,
}

#[derive(Debug)]
pub enum StatementKind {
    Expression(Expression),
}

#[derive(Debug)]
pub struct Expression {
    kind: ExpressionKind,
}

#[derive(Debug)]
pub enum ExpressionKind {
    NumberExpression(u128),
    PlusExpression(u128, u128),
    MinusExpression(u128, u128),
}

impl Parser {
    pub fn new(i: Vec<Token>) -> Self {
        let input = i
            .into_iter()
            .filter(|t| t.kind != TokenKind::Whitespace)
            .collect();
        Self {
            input,
            ..Default::default()
        }
    }

    pub fn parse(&mut self) -> Result<(), &'static str> {
        while let Ok(t) = self.consume() {
	     match t.kind {				 
                 TokenKind::Err => return Err("Error"),
                 TokenKind::Exit => continue,		 
                 _ => self.parse_expression(t)?,	 
             }					 
        }
        for s in &self.output {
            println!("Statement: {s:?}");
        }
        Ok(())
    }

    fn parse_expression(&mut self, t: Token) -> Result<(), &'static str> {
        match t.kind {
            TokenKind::Number(num) => {
                let one_ahead = self.peek(0);
		println!("{one_ahead:?}");
                if one_ahead.kind == TokenKind::Minus || one_ahead.kind == TokenKind::Plus {
		    println!("is a ");
                    let op = self.consume()?;
                    let t2 = self.consume()?;
                    let num2 = match t2.kind {
                        TokenKind::Number(num2) => num2,
                        _ => return Err("invalid Token"),
                    };
                    let kind = match op.kind {
                        TokenKind::Plus => ExpressionKind::PlusExpression(num, num2),
                        TokenKind::Minus => ExpressionKind::MinusExpression(num, num2),
                        _ => return Err("idfk"),
                    };
                    self.output.push(Statement {
                        kind: StatementKind::Expression(Expression { kind }),
                    });
                } else {
                    self.output.push(Statement {
                        kind: StatementKind::Expression(Expression {
                            kind: ExpressionKind::NumberExpression(num),
                        }),
                    });
                }
            }
            TokenKind::Plus => todo!("{t:?}"),
            TokenKind::Minus => todo!(),
            _ => {}
        }

        Ok(())
    }

    fn peek(&self, offset: usize) -> &Token {
        let mut i = self.pos + offset;
        if self.input.len() <= i {
            i = self.pos;
        }
        &self.input[i]
    }

    fn consume(&mut self) -> Result<Token, &'static str> {
        if self.pos >= self.input.len() {
            return Err("Not a valid index");
        }
        let t = self.input[self.pos].clone();
        self.pos += 1;
        Ok(t)
    }
}
