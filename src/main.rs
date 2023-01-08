use std::io;

mod hashmap_practice;
mod mean_median_mode;
mod string_practice;
mod vec_practice;
mod piglatin;

use hashmap_practice::hashmap_examples;
use mean_median_mode::{better_mode, mean, median, mode};
use string_practice::string_examples;
use vec_practice::vector_practice;
use piglatin::pig_latin;

fn main() {
    vector_practice();
    string_examples();
    hashmap_examples();

    let mut nums: Vec<i32> = vec![22, 33, 33, 44, 1, 7, 13, 15, 15, 15, 12];
    
    println!("Average: {}", mean(&nums));
    println!("Median: {}", median(&mut nums));
    println!("Mode: {}", mode(&nums));
    println!("Better Mode: {:?}", better_mode(&nums));

    println!("Enter a word to change to piglatin");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    println!("{}", pig_latin(&input));
    // println!("{}", better_pig_latin(&input));
}
