pub fn reference() {
    /*
     Moving ownership into a function and back out -> tedious
     Thus we use reference -> using variables without taking ownership.
    */
    let s1 = String::from("Hello");
    let (s2, len) = calculate_len(s1);
    println!("{},{}", s2, len)
}

fn calculate_len(string: String) -> (String, usize) {
    let len = string.len();
    return (string, len);
}
