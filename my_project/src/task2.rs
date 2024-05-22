//create a program that takes in an input on the terminal (hint: input should be an integer , print out from 0 to the input(see a loop, for in and while loop!)
use std::io;

fn main() {

    println!("Please enter a number:");

    // Read the user's input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Convert the input to an integer
    let number: i32 = input.trim().parse().unwrap();

    // Print out numbers from 0 to the input using a for loop
    println!("Using a for loop:");
    for i in 0..=number {
        println!("{}", i);
    }

    // Print out numbers from 0 to the input using a while loop
    println!("Using a while loop:");
    let mut i = 0;
    while i <= number {
        println!("{}", i);
        i += 1;
    }
}