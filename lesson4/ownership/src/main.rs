fn main() {
    let x = 32;
    let y = x; // copy
    println!("Ownership of x drops and comes to y: {}", y);

    let s1 = String::from("Hello");
    // let s2 = s1; // Move
    // let s2 = s1.clone(); // Clone

    // println!("Value: {}", s2);

    takes_owneship(s1);
    println!("After {}", s1); //S1's value already moved
}

fn takes_owneship(our_string: String) {
    println!("The value is: {}", our_string)
}
