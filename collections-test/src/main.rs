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
    ]
}
