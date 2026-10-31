use clap::Parser as CliParser;
use std::{io::{self, Write}};

pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod eval;

use error::CliCalcError;
use lexer::Lexer;
use parser::Parser;
use eval::eval_ast;

#[derive(CliParser, Debug)]
#[command(author, version, about = "CLI calc for learning")]
struct CliArgs {
    #[arg(num_args(0..))]
    expression: Vec<String>
}

fn main() {
    let args = CliArgs::parse();

    if !args.expression.is_empty() {
        let expr = args.expression.join(" ");
        match run_calculator(&expr) {
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
            match run_calculator(input) {
                Ok(result) => println!("Result: {}", result),
                Err(err) => println!("Error: {}", err),
            }
        }
    }
}

fn run_calculator(expr_str: &str) -> Result<f64, CliCalcError> {
    let mut lexer = Lexer::new(expr_str);
    let tokens = lexer.tokenize()?;

    let mut parser = Parser::new(tokens);
    eval_ast(&parser.parse()?)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_calc_precedence() {
        assert_eq!(run_calculator("2 + 3 * 4"), Ok(14.0));
        assert_eq!(run_calculator("(2 + 3) * 4"), Ok(20.0));
    }

    #[test]
    fn test_ast_calc_unary_minus() {
        assert_eq!(run_calculator("-5 + 3"), Ok(-2.0));
        assert_eq!(run_calculator("--5"), Ok(5.0));
    }

    #[test]
    fn test_ast_divide_by_zero() {
        assert_eq!(run_calculator("10 / (2 - 2)"), Err(CliCalcError::DivideByZero));
    }
}