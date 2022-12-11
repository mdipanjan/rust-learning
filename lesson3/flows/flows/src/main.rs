fn main() {
    //Control Flow
    let mut counter = 0;
    loop {
        counter += 1;
        if counter <= 10 {
            println!("Counter: {}", counter);
        } else {
            break;
        }
    }
    println!("=========================");
    // For in loop
    let arr = [20, 30, 40];
    for element in arr.iter() {
        println!("Element: {}", element);
    }
    println!("=========================");
    // loop through sequence
    for num in 50..59 {
        println!("The sequence number is: {}", num);
    }
}
