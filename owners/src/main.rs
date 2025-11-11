fn main() {
    // let mut s = String::from("hello");
    // println!("{s}");
    // s.push_str(", world!");
    // println!("{s}");
    //
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1 = {s1}, s2 = {s2}");


    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
    // 但 i32 是 Copy 的，
    println!("{}", x);              // 所以在后面可继续使用 x

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of {s2} is {len}.");

    let str = String::from("hello");
    let len= calculate_lengthII(&str);

    let mut s = String::from("hello");
    change(&mut s);

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("r1 is: {r1}");
    }
    let r2 = &mut s;
    println!("r2 is: {r2}");

    let s = String::from("hello");
    let slice = &s[0..2];
    println!("slice is: {slice}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_lengthII(s: &String) -> usize {
    s.len()
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{some_string}");
} // 这里，some_string 移出作用域并调用 `drop` 方法。
// 占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{some_integer}");
} // 这里，some_integer 移出作用域。没有特殊之处
