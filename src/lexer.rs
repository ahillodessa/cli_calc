use crate::ast::Token;
use crate::error::CliCalcError;

use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl <'a>Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, CliCalcError> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(&ch) = self.input.peek() {
            match ch {
                ' ' | '\t' | '\r' | '\n' => {
                    self.input.next();
                }
                '+' => {tokens.push(Token::Plus); self.input.next();},
                '-' => {tokens.push(Token::Minus); self.input.next();},
                '/' => {tokens.push(Token::Divide); self.input.next();},
                '*' => {tokens.push(Token::Multiply); self.input.next();},
                '(' => {tokens.push(Token::LParen); self.input.next();},
                ')' => {tokens.push(Token::RParen); self.input.next();},
                '0'..='9' | '.' => {
                    let num = self.read_numer()?;
                    tokens.push(Token::Number(num));
                }
                _ => {
                    return Err(CliCalcError::UnknownOperator(ch.to_string()));
                }
            }
        }
        Ok(tokens)
    }

    pub fn read_numer(&mut self) -> Result<f64, CliCalcError> {
        let mut num_str = String::new();
        while let Some(&ch) = self.input.peek() {
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
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_no_spaces() {
        let mut lexer = Lexer::new("2+3.5*4");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(2.0),
                Token::Plus,
                Token::Number(3.5),
                Token::Multiply,
                Token::Number(4.0)
            ]
        );
    }

    #[test]
    fn test_tokenize_with_parentheses() {
        let mut lexer = Lexer::new("(10 / 2) - 1");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Number(10.0),
                Token::Divide,
                Token::Number(2.0),
                Token::RParen,
                Token::Minus,
                Token::Number(1.0)
            ]
        );
    }
}