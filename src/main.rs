use clap::Parser;
use std::{io::{self, Write}};

pub mod ast;
pub mod error;
pub mod lexer;

use ast::Token;
use error::CliCalcError;
use lexer::Lexer;

#[derive(Parser, Debug)]
#[command(author, version, about = "CLI calc for learning")]
struct CliArgs {
    #[arg(num_args(0..))]
    expression: Vec<String>
}

fn main() {
    let args = CliArgs::parse();

    // if let Some(expr) = args.expression {
    if !args.expression.is_empty() {
        let expr = args.expression.join(" ");
        match eval_complex(&expr) {
            Ok(res) => println!("{}", res),
            Err(err) => println!("{}", err)
        }
    } else {
        println!("Calculator is ready for work");
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("Unable to read");
                break;
            }
            let input = input.trim();
            if input.is_empty() {
                println!("Go to next operation");
                continue;
            }
            match eval_complex(input) {
                Ok(result) => println!("Result: {}", result),
                Err(err) => println!("Error: {}", err),
            }
        }
    }
}

fn precedence(operand: &Token) -> i32{
    match operand {
        Token::Plus | Token::Minus => 1,
        Token::Multiply | Token::Divide => 2,
        _ => 0,
    }
}

fn apply_operand(numbers: &mut Vec<f64>, operand: &Token) -> Result<(), CliCalcError>{
    let right_val: f64 = numbers.pop().ok_or(CliCalcError::InvalidValue)?;
    let left_val: f64 = numbers.pop().ok_or(CliCalcError::InvalidValue)?;

    let res: f64 = match operand {
        Token::Plus => left_val + right_val,
        Token::Minus => left_val - right_val,
        Token::Multiply => left_val * right_val,
        Token::Divide => {
            if right_val == 0.0 {
                return Err(CliCalcError::DivideByZero);
            } 
            left_val / right_val
        }
        _ => return Err(CliCalcError::UnknownOperator(format!("{:?}", operand))),
    };

    numbers.push(res);
    Ok(())
}

fn eval_complex(expr: &str) -> Result<f64, CliCalcError> {
    let mut numbers: Vec<f64> = Vec::new();
    let mut operands: Vec<Token> = Vec::new();

    let mut lexer = Lexer::new(expr);
    let tokens = lexer.tokenize()?;

    for token in tokens {
        match token {
            Token::Number(num) => numbers.push(num),
            Token::LParen => operands.push(token),
            Token::RParen => {
                let mut found_lparen = false;
                while let Some(top_op) = operands.pop() {
                    if top_op == Token::LParen {
                        found_lparen = true;
                        break;
                    }
                    apply_operand(&mut numbers, &top_op)?;
                }
                if !found_lparen {
                    return Err(CliCalcError::InvalidExpression)
                }
            }
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide => {
                while let Some(top_op) = operands.last() {
                    if precedence(top_op) >= precedence(&token) {
                        let top_op = operands.pop().unwrap();
                        apply_operand(&mut numbers, &top_op)?;
                    } else {
                        break;
                    }
                }
                operands.push(token);
            }
        }
    }

    while let Some(operand) = operands.pop() {
        if operand == Token::LParen || operand == Token::RParen {
            return Err(CliCalcError::InvalidExpression)
        }
        apply_operand(&mut numbers, &operand)?;
    }

    if numbers.len() == 1 {
        Ok(numbers[0])
    } else {
        Err(CliCalcError::InvalidExpression)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_calc_no_spaces() {
        // Проверяем, что теперь всё отлично считается БЕЗ пробелов!
        assert_eq!(eval_complex("2+3*4"), Ok(14.0));
    }

    #[test]
    fn test_lexer_calc_with_parentheses() {
        // Проверяем поддержку скобок, которую мы заложили в лексер и стек
        assert_eq!(eval_complex("(2+3)*4"), Ok(20.0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(eval_complex("10/(5-5)"), Err(CliCalcError::DivideByZero));
    }

    #[test]
    fn test_precedence() {
        assert_eq!(precedence(&Token::Plus), 1);
        assert_eq!(precedence(&Token::Minus), 1);

        assert_eq!(precedence(&Token::Multiply), 2);
        assert_eq!(precedence(&Token::Divide), 2);

        assert_eq!(precedence(&Token::LParen), 0);
        assert_eq!(precedence(&Token::RParen), 0);
        // assert_eq!(precedence(&Token::Number(10), 0);
    }
}