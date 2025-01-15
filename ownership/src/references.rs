/*
########## The rules of Referencing ###################

1. At any given time, there should only be a single mutable 
reference and any number of immutable reference.

2. References must alwways be valid.
*/

pub fn noob() {
    /*
     Moving ownership into a function and back out -> tedious
     Thus we use reference -> using variables without taking ownership.
    */
    let s1 = String::from("Hello");
    // This method is a pain. Here we need the calculate_len method to return
    // both the string and length of it instead of the length only to avoid the
    // error thrown by the borrow checker. So we use reference.
    let (s2, len) = calculate_len(s1);
    println!("{},{}", s2, len)
}

fn calculate_len(string: String) -> (String, usize) {
    let len = string.len();
    return (string, len);
}

pub fn reference() {
    let s1 = String::from("Hello");
    let len = calculate_len_mod(&s1); // This is the reference to s1.
                                      // So the reference go from string -> s1 -> "Hello"
    println!("{},{}", s1, len)
}

// Passing in function parameters as reference is known as Borrowing.
// By passing the reference, we dont take the ownership. References are immutable by default.
fn calculate_len_mod(string: &String) -> usize {
    // string.push_str("blahblah"); Throws an error since reference is immutable.
    let len = string.len();
    return len;
}

// But what if we need to change the value??? There's a way.

pub fn reference_pro() {
    let mut s1 = String::from("Hello");
    let mod_s1 = change(&mut s1);
    println!("Original -> {}, \n Modified -> {}", s1, mod_s1);
}

fn change(string: &mut String) -> String {
    string.push_str("-hehehe");
    string.clone()
}
