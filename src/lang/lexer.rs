#![allow(dead_code)]

#[derive(Debug, Default, Clone)]
pub struct Lexer {
    pub(crate) input: String,
    pub(crate) pos: usize,
    pub(crate) tokens: Vec<Token>,
}

#[derive(Debug, Default, Clone)]
pub struct Token {
    pub(crate) kind: TokenKind,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum TokenKind {
    Number(u128),
    Plus,
    Minus,
    #[default]
    Err,
    Whitespace,
    Exit,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            ..Default::default()
        }
    }

    pub fn lex_input(&mut self) -> Result<(), &'static str> {
        while self.pos < self.input().len() {
            let t = self.consume();
            self.tokens.push(t);
        }
        Ok(())
    }

    pub fn consume(&mut self) -> Token {
        let current = self.current();
        let mut t: Token;
        if current.is_ascii_digit() {
            t = self.consume_number();
            return t;
        } else if current.is_whitespace() {
            t = Token {
                kind: TokenKind::Whitespace,
            };
        } else {
            t = self.consume_char();
        }
        self.pos += 1;
        t
    }

    fn consume_number(&mut self) -> Token {
        let mut buf = String::new();
        buf.push(self.current());
        self.pos += 1;
        while self.input.len() > self.pos && self.input()[self.pos].is_numeric() {
            buf.push(self.current());
            self.pos += 1;
        }

        Token {
            kind: TokenKind::Number(buf.parse().expect("fuck this thing")),
        }
    }

    fn consume_char(&mut self) -> Token {
        let c = self.current();

        let kind: TokenKind = match c {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            _ => return self.consume_word(),
        };
        Token { kind }
    }

    fn consume_word(&mut self) -> Token {
        let mut buf = String::new();
        while self.input.len() > self.pos && self.input()[self.pos].is_ascii() {
            buf.push(self.current());
            self.pos += 1;
        }
        let kind = match buf.to_lowercase().as_str() {
            "exit" => TokenKind::Exit,
            _ => TokenKind::default(),
        };
        Token { kind }
    }

    fn current(&self) -> char {
        self.input()[self.pos]
    }

    fn input(&self) -> Vec<char> {
        self.input.chars().collect()
    }
}
