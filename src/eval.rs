use crate::error::CliCalcError;
use crate::ast::*;

pub fn eval_ast(expr: &Expr) -> Result<f64, CliCalcError> {
    match expr {
        Expr::Number(num) => Ok(*num),
        
        Expr::UnaryOp{operand, expr} => {
            let val = eval_ast(expr)?;
            match operand {
                TokenKind::Minus => Ok(-val),
                _ => Err(CliCalcError::UnknownOperator(format!("{:?}", operand))),
            }
        },

        Expr::BinOp { left, operand, right } => {
            let left_val = eval_ast(left)?;
            let right_val = eval_ast(right)?;

            match operand {
                TokenKind::Plus => Ok(left_val + right_val),
                TokenKind::Minus => Ok(left_val - right_val),
                TokenKind::Multiply => Ok(left_val * right_val),
                TokenKind::Divide => {
                    if right_val == 0.00 {
                        return Err(CliCalcError::DivideByZero);
                    }
                    Ok(left_val / right_val)
                }
                _ => Err(CliCalcError::UnknownOperator(format!("{:?}", operand))),
            }
        }
    }
}