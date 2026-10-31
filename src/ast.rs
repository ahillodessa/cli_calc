#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,  // (
    RParen,  // )
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f64),
    
    BinOp {
        left: Box<Expr>,
        operand: TokenKind,
        right: Box<Expr>,
    },

    UnaryOp {
        operand: TokenKind,
        expr: Box<Expr>,
    }
}