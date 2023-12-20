// src/lexer.rs

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Dot,
    Dash,
    Space,
    Eof,
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input,
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(ch) = self.current_char() {
            match ch {
                '.' => {
                    self.advance();
                    return Token::Dot;
                }
                '-' => {
                    self.advance();
                    return Token::Dash;
                }
                ' ' => {
                    self.advance();
                    return Token::Space;
                }
                _ => {
                    // Yorum satırı olarak işaretlenen eğik çizgi (/) karakterini tanıma
                    if ch == '/' {
                        self.advance();
                        continue;
                    }
                    self.advance();
                }
            }
        }
        Token::Eof
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.position..)?.chars().next()
    }

    fn advance(&mut self) {
        if let Some(ch) = self.current_char() {
            self.position += ch.len_utf8();
        }
    }
}
