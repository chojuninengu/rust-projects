use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CalculatorError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Division by zero")]
    DivisionByZero,
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}

pub struct Calculator;

impl Calculator {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    pub fn divide(a: i32, b: i32) -> Result<i32, CalculatorError> {
        if b == 0 {
            return Err(CalculatorError::DivisionByZero);
        }
        Ok(a / b)
    }

    pub fn parse_input(input: &str) -> Result<i32, CalculatorError> {
        input.trim().parse().map_err(|_| {
            CalculatorError::InvalidInput(format!("Could not parse '{}' as a number", input))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_add() {
        assert_eq!(Calculator::add(2, 3), 5);
        assert_eq!(Calculator::add(-1, 1), 0);
        assert_eq!(Calculator::add(0, 0), 0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(Calculator::subtract(5, 3), 2);
        assert_eq!(Calculator::subtract(1, 1), 0);
        assert_eq!(Calculator::subtract(0, 5), -5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(Calculator::multiply(2, 3), 6);
        assert_eq!(Calculator::multiply(-2, 3), -6);
        assert_eq!(Calculator::multiply(0, 5), 0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(Calculator::divide(6, 2).unwrap(), 3);
        assert_eq!(Calculator::divide(-6, 2).unwrap(), -3);
        assert!(Calculator::divide(5, 0).is_err());
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(Calculator::parse_input("42").unwrap(), 42);
        assert_eq!(Calculator::parse_input("-42").unwrap(), -42);
        assert!(Calculator::parse_input("not a number").is_err());
    }

    proptest! {
        #[test]
        fn test_add_property(a: i32, b: i32) {
            let result = Calculator::add(a, b);
            assert_eq!(result, a + b);
        }

        #[test]
        fn test_subtract_property(a: i32, b: i32) {
            let result = Calculator::subtract(a, b);
            assert_eq!(result, a - b);
        }

        #[test]
        fn test_multiply_property(a: i32, b: i32) {
            let result = Calculator::multiply(a, b);
            assert_eq!(result, a * b);
        }

        #[test]
        fn test_divide_property(a: i32, b in 1..=i32::MAX) {
            let result = Calculator::divide(a, b).unwrap();
            assert_eq!(result, a / b);
        }
    }
}
