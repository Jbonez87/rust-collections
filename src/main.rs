use std::{io, collections::HashMap};
use colored::*;

/*
  These are all of the internal modules to print out the collection
  types. 
 */
mod hashmap_practice;
mod mean_median_mode;
mod string_practice;
mod vec_practice;
mod piglatin;
mod employees;

/*
  These are all the functions used from the modules.
 */
use hashmap_practice::hashmap_examples;
use mean_median_mode::{better_mode, mean, median, mode};
use string_practice::string_examples;
use vec_practice::vector_practice;
use piglatin::pig_latin;
use employees::{exec_add, exec_get};


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
    
    println!("{}", pig_latin(&input).cyan());

    let mut dict = HashMap::new();
    let mut command = String::new();

    loop {
        print!("Enter command: ");
        io::stdin()
          .read_line(&mut command)
          .expect("Failed to read from stdin");
        let tokens = command
          .split_whitespace()
          .collect::<Vec<_>>();
        if let Some(&action) = tokens.first() {
          if let Err(msg) = match action {
            "ADD" => exec_add(&tokens, &mut dict),
            "GET" => exec_get(&tokens, &dict),
            "EXIT" => break,
            _ => Err("Invalid Command.")
          } {
            println!("Error: {}", msg.red());
          }
        };
        command.clear();
    }
    println!("Bye.")
}
