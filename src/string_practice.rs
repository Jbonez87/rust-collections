pub struct Locolib {
    pub text: String,
}

impl Locolib {
    pub fn get_text(&self) {
        println!("Locolib text is: {}", &self.text);
    }
}

pub fn string_examples() {
    let mut s = String::from("lo");

    s.push('l');

    println!("s is: {}", s);

    let data = "initial contents";

    let s = data.to_string();

    println!("s is: {s}");

    let s = "initial contents".to_string();

    println!("s is: {s}");

    let mut s = String::from("Foo");

    s.push_str(" bar!");

    println!("s is: {s}");

    let mut s1 = String::from("foo");

    let s2 = "bar";

    s1.push_str(s2);

    println!("s2 is {}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    println!("s3 is: {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("s is now: {s}");

    let s1: String = String::from("tic");
    let s2: String = String::from("tac");
    let s3: String = String::from("toe");
    let locolib = Locolib {
        text: String::from("!"),
    };

    let s: String = format!("{}-{}-{}{}", s1, s2, s3, locolib.text);

    println!("s is now formatted as: {s}");

    locolib.get_text();

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    println!("s is now a substring: {}", s);
    println!("Curly braces must be escaped in Rust: {{}}");
    string_iteration_example();
}

fn string_iteration_example() {
    for c in "Зд".chars() {
        println!("{}", c);
    }

    for b in "Зд".bytes() {
        println!("{}", b);
    }
}
