#![allow(unused_mut)]
use std::io::{stdin, stdout, Stdout, Write};

use lang::{
    lexer::{Lexer, TokenKind},
    parser::Parser,
};

mod lang;

fn main() -> Result<(), &'static str> {
   let mut quit = false;
    while !quit {
        let mut stdout = stdout();
        let mut stdin = stdin();
        print_prompt(stdout);

        let mut buf = String::new();
        let _ = stdin.read_line(&mut buf);
        buf = buf.trim().to_string();

        let mut lexer = Lexer::new(buf);
        let _ = lexer.lex_input()?;
        for t in &lexer.tokens {
            if t.kind == TokenKind::Exit {
                quit = true;
            }
        }
	let mut parser = Parser::new(lexer.tokens);
        parser.parse()?;
    }

    Ok(())
}

fn print_prompt(mut stdout: Stdout) {
    let _ = stdout.write_all(b"> ");
    let _ = stdout.flush();
}
