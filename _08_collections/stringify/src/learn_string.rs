pub fn learn_() -> (){
    // String stored as utf-8 
    // in utf-8, each character could be stored in varying lengths of bytes.
    let mut s1: String = String::new(); // empty string
    let s2 = "Hello world"; // String slice 
    let s3 = s2.to_string();// Type cast to string
    let mut s4 = String::from("Hello world");// String

    // Appending to a string
    s4.push_str("blah");
    s1.push('!'); // Use single quotes for characters

    let final_string = format!("{} {}", s4, s3);
    println!("{}", final_string); // Here, we can use s4 after this point
    // because format macro doesnt take ownership

    let final_string = s4 + &s3;
    println!("{}", final_string); // Here, we can't use s4 after this point. Error adikkum
    
    println!("Executing index function...");
    indexing();
}

fn indexing() -> (){
    use unicode_segmentation::UnicodeSegmentation;
    let hello = String::from("നമസ്കാരം");
    println!("{}", hello);
    //*  let c = hello[0]; This is not possible in Rust because it points to the fiirst byte of Hello,
    //*  It doesnt guarantee that the first byte completely contains the first letter. In UTF-8, the characters 
    //* could be 1-4 bytes long.

    //? Strings consists of either Bytes, Scalar values or Graphemes clusters
    
    // Iterating through bytes
    for i in hello.bytes(){
        println!("{}", i)
    }
    println!("");

    // Iterating through scalar values
    for i in hello.chars(){
        println!("{}", i)
    }
    println!("");

    // Iterating through grapheme cluster. //? You need unicode-segmentation crate.
    for i in hello.graphemes(true){
        println!("{}", i)
    }
}