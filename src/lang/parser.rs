#![allow(dead_code)]
use super::lexer::*;

#[derive(Debug, Default)]
pub struct Parser {
    pub(crate) input: Vec<Token>,
    pub(crate) output: Vec<Statement>,
    pub(crate) pos: usize,
}

#[derive(Debug)]
pub struct Statement {
    pub(crate) kind: StatementKind,
}

#[derive(Debug)]
pub enum StatementKind {
    Expression(Expression),
}

#[derive(Debug)]
pub struct Expression {
    pub(crate) kind: ExpressionKind,
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
        Ok(())
    }

    fn parse_expression(&mut self, t: Token) -> Result<(), &'static str> {
        match t.kind {
            TokenKind::Number => {
		let v = t.value.unwrap();
		let num = match v {
		    Value::Number(num) => num,
		    _ => return  Err("error"),
		};
                let one_ahead = self.peek(0);
                if one_ahead.kind == TokenKind::Minus || one_ahead.kind == TokenKind::Plus {
                    let op = self.expect(vec![TokenKind::Plus, TokenKind::Minus])?;
		    let v2 = match self.expect(vec![TokenKind::Number])?.kind {
			TokenKind::Number => self.peek(0).value.clone().unwrap(),
			_ => return Err("WTF"),
			    
		    };
		    
		    let num2 = match v2 {
			Value::Number(num) => num,
			_ => return  Err("err"),
		    };
                    		    
                    let kind = match op.kind {
                        TokenKind::Plus => ExpressionKind::PlusExpression(num,num2),
                        TokenKind::Minus => ExpressionKind::MinusExpression(num,num2),
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
            TokenKind::Plus => return Err("todo"),
            TokenKind::Minus => return Err("todo"),
            _ => {}
        }

        Ok(())
    }
    
    fn expect(&mut self, tks: Vec<TokenKind>) -> Result<Token, &'static str> {
	let tc = self.consume()?;
	let mut found = false;
	for tk in tks {
	    if tk == tc.kind {
		found = true;
	    }
	}
	if found {
	    Ok(tc)
	} else {
	    Err("expected smth diff")
	}

    }


    fn peek(&self, offset: usize) -> &Token {
        let mut i = self.pos + offset;
        if self.input.len() <= i {
            i = self.pos - 1;
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
