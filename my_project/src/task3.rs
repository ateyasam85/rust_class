//create a programthat takes in an input from the terminal and subtract, addition, multiply and divide (any number)
use std::io::{self, Write};

fn main() {
    let mut num1 = String::new();
    print!("Enter the first number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut num1).unwrap();
    let num1: f32 = num1.trim().parse().unwrap();

    let mut num2 = String::new();
    print!("Enter the second number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut num2).unwrap();
    let num2: f32 = num2.trim().parse().unwrap();

    println!("Addition: {}", num1 + num2);
    println!("Subtraction: {}", num1 - num2);
    println!("Multiplication: {}", num1 * num2);
    if num2 != 0.0 {
        println!("Division: {}", num1 / num2);
    } else {
        println!("Cannot divide by zero");
    }
}

