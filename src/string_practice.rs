use unicode_segmentation::UnicodeSegmentation;

pub struct Template {
    pub text: String,
}

impl Template {
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
    let template = Template {
        text: String::from("!"),
    };

    let s: String = format!("{}-{}-{}{}", s1, s2, s3, template.text);

    println!("s is now formatted as: {s}");

    template.get_text();

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    println!("s is now a substring: {}", s);
    println!("Curly braces must be escaped in Rust: {{}}");
    string_iteration_example();

    let mut s = String::from("Hello");
    s.push_str(", John");
    s.push('!');

    println!("{s}");

    string_value_types();
}

fn string_iteration_example() {
    for c in "Зд".chars() {
        println!("{}", c);
    }

    for b in "Зд".bytes() {
        println!("{}", b);
    }
}

fn string_value_types() {
    let s = String::from("नमस्ते");

    // Bytes
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // Scalar values
    // ['न', 'म', 'स', '्', 'त', 'े']

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // Graphene Clusters
    // ["न", "म", "स्", "ते"]

    for g in "नमस्ते".graphemes(true) {
        println!("{}", g);
    }

    println!("{s}");
}
