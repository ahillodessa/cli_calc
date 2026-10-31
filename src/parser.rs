use crate::ast::{Expr, Token, TokenKind};
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
            let extra_token = &self.tokens[self.position];
            return Err(CliCalcError::ParserError {
                pos: extra_token.pos,
                message: format!("unexpected token{:?}", extra_token.kind)});
        }
        Ok(ast)
    }

    fn pase_expr(&mut self) -> Result<Expr, CliCalcError> {
        let mut left = self.parse_term()?;

        while let Some(token) = self.current_token() {
            match token.kind {
                TokenKind::Plus | TokenKind::Minus => {
                    self.position += 1;
                    let right = self.parse_term()?;
                    left = Expr::BinOp { 
                        left: Box::new(left),
                        operand: token.kind,
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
            match token.kind {
                TokenKind::Multiply | TokenKind::Divide => {
                    self.position += 1;
                    let right = self.pase_factor()?;
                    left = Expr::BinOp { 
                        left: Box::new(left),
                        operand: token.kind,
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

        match token.kind {
            TokenKind::Number(num) => {
                self.position += 1;
                Ok(Expr::Number(num))
            }
            TokenKind::Minus => {
                self.position += 1;
                let expr = self.pase_factor()?;
                Ok(Expr::UnaryOp { operand: TokenKind::Minus, expr: Box::new(expr) })
            }
            TokenKind::LParen => {
                self.position += 1;
                let expr = self.pase_expr()?;
                if self.current_token().map(|t| t.kind) == Some(TokenKind::RParen) {
                    self.position += 1;
                    Ok(expr)
                } else {
                    Err(CliCalcError::ParserError {
                        pos: token.pos,
                        message: "unmatched opening parenthesis '('".to_string()})
                }
            }
            _ => Err(CliCalcError::ParserError {
                pos: token.pos,
                message: format!("unexpected token{:?}", token)
            })
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
                operand: TokenKind::Plus,
                right: Box::new(Expr::Number(3.0))
            }
        );
    }
}