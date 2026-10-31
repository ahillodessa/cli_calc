use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CliCalcError {
    #[error("Parser error at position: {pos}:{message}")]
    ParserError{pos: usize, message: String},

    #[error("Invalid expression format")]
    InvalidExpression,

    #[error("Cannot divide by zero")]
    DivideByZero,

    #[error("Unknown operator: {0}")]
    UnknownOperator(String),

    #[error("Invalid value: stack is empty")]
    InvalidValue,

    #[error("Cannot parse digit: {0}")]
    ParseDigitError(String),
}