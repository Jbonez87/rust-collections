#[derive(Debug)]
pub enum SpreadsheetCell {
    Int(i64),
    Float(f64),
    Text(String),
}

pub fn vector_practice() {
    // example using Vector literal
    let v: Vec<i64> = Vec::new();
    println!("Empty vec: {:?}", v);

    // example using vec! macro
    let v: Vec<i64> = vec![2, 4, 6];
    println!("Macro vec: {:?}", v);

    // mutable Vector example
    let mut v: Vec<i64> = Vec::new();

    v.push(5);
    v.push(30);
    v.push(8);
    v.push(40);
    v.push(9);

    // reads entire vector
    println!("{:?}", v);

    let v: Vec<i64> = vec![5, 10, 15, 20, 25];

    let third: &i64 = &v[2];
    println!("The third element is: {}", third);

    let third: Option<&i64> = v.get(2);
    match third {
        Some(third) => println!("The third element is: {}", third),
        None => println!("There is no third element."),
    }

    let v: Vec<i64> = vec![5, 6, 7, 8, 9];

    // iterating example
    for i in &v {
        println!("Value is: {}", i);
    }

    let mut v: Vec<i64> = vec![100, 57, 32];

    // iterating mutable vector example
    for val in &mut v {
        *val += 50;
        println!("Value is now: {}", val);
    }

    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(55),
        SpreadsheetCell::Float(8.9),
        SpreadsheetCell::Text(String::from("Blue")),
    ];

    for val in &row {
        println!("Val is: {:?}", val);
    }

    // map example
    // v.iter().map(|&x| {
    //   let new = x - 5;
    //   println!("x is now: {}", new);
    //   return new;
    // }).collect::<Vec<i64>>();
}
