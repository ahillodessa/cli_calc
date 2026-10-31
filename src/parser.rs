use crate::ast::{Expr, Token};
use crate::error::CliCalcError;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self{tokens, position: 0}
    }

    pub fn parse(&mut self) -> Result<Expr, CliCalcError> {
        let ast = self.pase_expr()?;
        if self.position < self.tokens.len() {
            return Err(CliCalcError::InvalidExpression);
        }
        Ok(ast)
    }

    fn pase_expr(&mut self) -> Result<Expr, CliCalcError> {
        let mut left = self.parse_term()?;

        while let Some(token) = self.current_token() {
            match token {
                Token::Plus | Token::Minus => {
                    self.position += 1;
                    let right = self.parse_term()?;
                    left = Expr::BinOp { 
                        left: Box::new(left),
                        operand: token,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, CliCalcError> {
        let mut left = self.pase_factor()?;

        while let Some(token) = self.current_token() {
            match token {
                Token::Multiply | Token::Divide => {
                    self.position += 1;
                    let right = self.pase_factor()?;
                    left = Expr::BinOp { 
                        left: Box::new(left),
                        operand: token,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn pase_factor(&mut self) -> Result<Expr, CliCalcError> {
        let token = self.current_token().ok_or(CliCalcError::InvalidExpression)?;

        match token {
            Token::Number(num) => {
                self.position += 1;
                Ok(Expr::Number(num))
            }
            Token::Minus => {
                self.position += 1;
                let expr = self.pase_factor()?;
                Ok(Expr::UnaryOp { operand: Token::Minus, expr: Box::new(expr) })
            }
            Token::LParen => {
                self.position += 1;
                let expr = self.pase_expr()?;
                if self.current_token() == Some(Token::RParen) {
                    self.position += 1;
                    Ok(expr)
                } else {
                    Err(CliCalcError::InvalidExpression)
                }
            }
            _ => Err(CliCalcError::InvalidExpression)
        }
    }

    fn current_token(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            Some(self.tokens[self.position].clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parser_simple() {
        let mut lexer = Lexer::new("2+3");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(
            ast,
            Expr::BinOp {
                left: Box::new(Expr::Number(2.0)),
                operand: Token::Plus,
                right: Box::new(Expr::Number(3.0))
            }
        );
    }
}