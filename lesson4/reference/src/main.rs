fn main() {
    let str1 = String::from("Dynamic String");
    let len1 = calculate_string_length(str1);
    println!("Length of the string: {}", len1);
    // The str1's value is moved into calculate_string_length function
    // Now if we try to access it's value, it'll throw error, and we can not borrow the value
    println!("Try access str1: {}", str1);

    // Now this can be resolved by -> reference
    let str2 = String::from("Dynamic String");
    // here str2 is paased inside calculate_string_length_with_reference as a reference
    // so the value isn't moved/ ownership is unchanged, and we can reuse str2's value
    calculate_string_length_with_reference(&str2);
    let len2 = calculate_string_length(str2);
    println!("Length of the string: {}", len2);
}
fn calculate_string_length(s: String) -> usize {
    let length_of_str: usize = s.len();
    return length_of_str;
}
fn calculate_string_length_with_reference(s: &String) -> usize {
    let length_of_str: usize = s.len();
    return length_of_str;
}
