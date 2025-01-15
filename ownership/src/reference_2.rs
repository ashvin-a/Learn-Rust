

pub fn mutable(){
    let mut s = String::from("Hello");
    
    let r1 = &s; 
    let r2 = &s; 
    // let r3 = &mut s; This line throws an error since there are
    // immutable references in this scope.
    
    println!("{}, {}", r1, r2);
    let r3 = &mut s; // This line wont throw an error 
    // since there are no immutable references at this point.(The scope of 
    // their ends above. )
    println!("{}", r3);


    // let reference_to_nothing = dangle();
    // println!("{}", reference_to_nothing);
}

// pub fn dangle() -> &String{
//     let s = String::from("Blah");
     // &s // This reference is only valid within the scope of this function.
     // Thus, when we call this function, we get an error.
// }