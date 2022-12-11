fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 7;
    println!("The new mutable value of x is: {}", x);
    // Constatnt in Rusty can not be mutated, need to be uppercased
    const Y: u32 = 10;
    // Data Types
    // Integers
    // Floating point numbers
    // Booleans
    // CHaracter

    //Integer: Compound Types
    let tup = ("user", 100);
    let (user, number) = tup;
    println!("number opf users: {}", number);

    // Arrays
    let error_codes = [404, 500];
    let not_found = error_codes[0];
    println!("{} Not found", not_found);

    // Founctions
    rust_fn();
    let sum = my_function(45, 45);
    println!("The sum is {}", sum);
}
fn rust_fn() {
    println!("This is a rust function");
}
fn my_function(x: i32, y: i32) -> i32 {
    let sum = x + y;
    return sum;
}
