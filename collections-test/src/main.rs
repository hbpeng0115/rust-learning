use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let mut v1 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v1[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v1.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let first = &v1[0];
    v1.push(6);
    // it will produce compile error
    // println!("The first element is: {first}");

    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        println!("{i}");
        *i += 50;
        println!("{i}");
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    println!("s1 is {s1}");
    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 5);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");
    for (key, value) in scores {
        println!("{key}: {value}");
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1
    }
    println!("{map:?}");
}
