use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let numbers: Vec<i32> = buffer.split_whitespace().map(|num| -> i32 { num.parse().unwrap() }).collect();
    let mut seen_frequencies = HashSet::new();
    let mut current_frequency = 0;
    seen_frequencies.insert(current_frequency);
    
    loop {
        for number in numbers.iter() {
            current_frequency += number;
            println!("Current frequency: {}", current_frequency);
            if seen_frequencies.contains(&current_frequency) {
                println!("First repeated frequency: {}", current_frequency);
                return
            }
            seen_frequencies.insert(current_frequency);
        }
    }
}
