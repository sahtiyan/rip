// src/parser.rs

use crate::lexer::{Lexer, Token};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        let mut parser = Parser {
            lexer,
            current_token: Token::Eof,
        };
        parser.advance_token();
        parser
    }

    fn advance_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> String {
        let mut output = String::new();
        loop {
            match self.current_token {
                Token::Dot => {
                    output.push('.');
                    self.advance_token();
                }
                Token::Dash => {
                    output.push('-');
                    self.advance_token();
                }
                Token::Space => {
                    output.push(' ');
                    self.advance_token();
                }
                Token::Eof => break,
            }
        }
        output
    }
}
