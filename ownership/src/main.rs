mod references;

fn main() {
    /*
        ******Ownership rules******
        1. Each value in Rust has a variable called the owner
        2. There can only be one owner at a time.
        3. When the owner goes out of scope, the value is dropped.
    */
    {
        let s = "string"; // Strong literals are stored directly in the memory and are fixed.
    } // s no longer available
    let x = 5;
    let y = x; // Copy

    let s1 = String::from("hello");

    // let s2 = s1; // Move. This throws an error from the borrow checker.
    let s2 = s1.clone(); // Clones the variable

    println!("{}", s1);

    let s: String = String::from("Testing");

    takes_ownership(s);
    // println!("{}",s); // Throws an error

    let x1 = String::from("hellop");
    let x2 = takes_and_gives(x1);
    println!("{}", x2);
    references::reference();
}

fn takes_ownership(string: String) {
    println!("{}", string);
}

fn takes_and_gives(string: String) -> String {
    string
}
