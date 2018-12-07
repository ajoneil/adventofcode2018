use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let numbers = buffer.split_whitespace().map(|num| -> i32 { num.parse().unwrap() });
    let sum: i32 = numbers.sum();
    println!("Sum: {}", sum);
}
