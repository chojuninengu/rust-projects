use calculator::{Calculator, CalculatorError};
use std::io::{self, Write};

fn main() -> Result<(), CalculatorError> {
    println!("\t\tCALCULATOR\n\n\tAVAILABLE ACTIONS\n1.Addition\n2.Subtraction\n3.Multiplication\n4.Division");

    print!("\n\t:: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let operation = input.trim();

    print!("Number 1: ");
    io::stdout().flush()?;
    let mut first = String::new();
    io::stdin().read_line(&mut first)?;
    let first_num = Calculator::parse_input(&first)?;

    print!("Number 2: ");
    io::stdout().flush()?;
    let mut second = String::new();
    io::stdin().read_line(&mut second)?;
    let second_num = Calculator::parse_input(&second)?;

    let result = match operation {
        "1" | "Addition" | "+" => Calculator::add(first_num, second_num),
        "2" | "Subtraction" | "-" => Calculator::subtract(first_num, second_num),
        "3" | "Multiplication" | "*" => Calculator::multiply(first_num, second_num),
        "4" | "Division" | "/" => Calculator::divide(first_num, second_num)?,
        _ => {
            println!("Invalid operation selected!");
            return Ok(());
        }
    };

    println!("\n\n\tResult: {}", result);
    Ok(())
}
