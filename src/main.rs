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

    let mut map = HashMap::<String, Vec<String>>::new();
    loop {
        let action = next_action();
        match action {
            Action::Add(employee, department) => {
                let employees = map.entry(department).or_insert(Vec::new());
                employees.push(employee);
            },
            Action::List(department) => {
                let employees = map.get_mut(&department);
                if let Some(employees) = employees  {
                    employees.sort_unstable();
                    println!("Department '{}' has employees {:?}", department, employees);
                } else {
                    println!("Department '{}' has has no employees yet", department);
                }
            },
            Action::Quit => {
                break;
            },
            Action::Unknown => {}
        }
    }
    println!("map: {:?}", map);
}

enum Action {
    Add(String, String),
    List(String),
    Quit,
    Unknown
}

fn next_action() -> Action {
    let mut input = String::new();
    println!("What do you want to do?\n\
    a $PERSON $DEPARTMENT => adds $PERSON to $DEPARTMENT\n\
    l $DEPARTMENT         => lists all members of $DEPARTMENT in alphabetic order\n\
    q                     => quits the app ");

    io::stdin()
          .read_line(&mut input)
          .expect("Failed to read line!");
    input_to_action(input)
}

fn input_to_action(s: String) -> Action {
    println!("{s}");
    let words: Vec<&str> = s.split_whitespace().collect();
    match words.as_slice() {
        [ "a", name, department ] =>  Action::Add(name.to_string(), department.to_string()),
        [ "l", department ] => Action::List(department.to_string()),
        [ "q" ] => Action::Quit,
        _ => Action::Unknown
    }
}
