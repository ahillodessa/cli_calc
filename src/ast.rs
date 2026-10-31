#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,  // (
    RParen,  // )
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f64),
    
    BinOp {
        left: Box<Expr>,
        operand: Token,
        right: Box<Expr>,
    },

    UnaryOp {
        operand: Token,
        expr: Box<Expr>,
    }
}