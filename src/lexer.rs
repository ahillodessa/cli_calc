use crate::ast::{Token, TokenKind};
use crate::error::CliCalcError;

use std::iter::Peekable;
use std::str::CharIndices;

pub struct Lexer<'a> {
    input: Peekable<CharIndices<'a>>,
}

impl <'a>Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.char_indices().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, CliCalcError> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(&(pos, ch)) = self.input.peek() {
            match ch {
                ' ' | '\t' | '\r' | '\n' => {
                    self.input.next();
                }
                '+' => { tokens.push(Token { kind: TokenKind::Plus, pos }); self.input.next(); }
                '-' => { tokens.push(Token { kind: TokenKind::Minus, pos }); self.input.next(); }
                '/' => { tokens.push(Token { kind: TokenKind::Divide, pos }); self.input.next(); }
                '*' => { tokens.push(Token { kind: TokenKind::Multiply, pos }); self.input.next(); }
                '(' => { tokens.push(Token { kind: TokenKind::LParen, pos }); self.input.next(); }
                ')' => { tokens.push(Token { kind: TokenKind::RParen, pos }); self.input.next(); }
                '0'..='9' | '.' => {
                    let num = self.read_numer()?;
                    tokens.push(Token{kind: TokenKind::Number(num), pos});
                }
                _ => {
                    return Err(CliCalcError::ParserError {
                        pos,
                        message: format!("unexpected character '{}'", ch),
                    });
                }
            }
        }
        Ok(tokens)
    }

    pub fn read_numer(&mut self) -> Result<f64, CliCalcError> {
        let mut num_str = String::new();
        while let Some(&(_, ch )) = self.input.peek() {
            match ch {
                '0'..='9' | '.' => {
                    num_str.push(ch);
                    self.input.next();
                }
                _ => break
            }
        }
        num_str
        .parse::<f64>()
        .map_err(|_| CliCalcError::ParseDigitError(num_str))
    }

}

#[cfg(test)]
fn test_tokenize_with_parentheses() {
    let mut lexer = Lexer::new("(10 / 2) - 1");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![
            Token { kind: TokenKind::LParen, pos: 0 },
            Token { kind: TokenKind::Number(10.0), pos: 1 },
            Token { kind: TokenKind::Divide, pos: 4 },
            Token { kind: TokenKind::Number(2.0), pos: 6 },
            Token { kind: TokenKind::RParen, pos: 7 },
            Token { kind: TokenKind::Minus, pos: 9 },
            Token { kind: TokenKind::Number(1.0), pos: 11 }
        ]
    );
}

#[test]
fn test_tokenize_unexpected_char() {
    let mut lexer = Lexer::new("2 + # 3");
    let result = lexer.tokenize();
    
    assert_eq!(
        result,
        Err(CliCalcError::ParserError {
            pos: 4,
            message: "unexpected character '#'".to_string()
        })
    );
}