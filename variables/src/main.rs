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
}