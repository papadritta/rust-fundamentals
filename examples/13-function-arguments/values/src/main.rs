/* Standart exercise

use std::io;
fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn main() {
    // There are no variadic arguments in Rust
    let numbers = [1, 2, 3, 4, 5];
    let result = sum(&numbers);
    println!("The sum is {}", result);
}
*/

/* Modification 1

use std::io;
fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn main() {
    println!("Enter the number of elements:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let num_elements: usize = input.trim().parse().expect("Invalid input");

    let mut numbers = Vec::new();
    for _ in 0..num_elements {
        println!("Enter a number:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let number: i32 = input.trim().parse().expect("Invalid input");
        numbers.push(number);
    }

    let result = sum(&numbers);
    println!("The sum is {}", result);
}
*/

// Modification 2
use std::io;

fn average(numbers: &[i32]) -> f64 {
    let sum: i32 = numbers.iter().sum();
    let count = numbers.len() as f64;
    sum as f64 / count
}

fn main() {
    println!("Enter the number of elements:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let num_elements: usize = input.trim().parse().expect("Invalid input");

    let mut numbers = Vec::new();
    for _ in 0..num_elements {
        println!("Enter a number:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let number: i32 = input.trim().parse().expect("Invalid input");
        numbers.push(number);
    }

    let result = average(&numbers);
    println!("The average is {}", result);
}
