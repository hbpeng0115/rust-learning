fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;

    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("const value is: {THREE_HOURS_IN_SECONDS}");

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x1, x2, x3) = tup;
    println!("The value of x2 is: {x2}");

    let first_element = tup.0;
    let second_element = tup.1;
    let third_element = tup.2;

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a_first = a[0];
    let a_second = a[1];

    another_function(5, 'h');

    let y: i32 = return_function(100);
    println!("The value of y is: {y}");

    branch_test();
    loop_test();
    label_loop_test();
    for_test();
    for_test_II();
}

fn for_test_II() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!");
}

fn for_test() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}

fn label_loop_test() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!()
}

fn loop_test() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn branch_test() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    }
    else {
        println!("condition was false");
    }
}

fn return_function(x: i32) -> i32 {
    // let res = x / 2;
    // return res;

    x / 2
}

fn another_function(x: i32, unit_label: char) {
    println!("Another function.");
    println!("The measurement value is: {x}, {unit_label}");
}