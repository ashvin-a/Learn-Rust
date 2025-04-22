//* Lets get to know about dangling references */
fn main() {
    let r: &i32;

    {
        let x = 5;
        r = &x; //? This will throw an error if r is mentioned outside this scope.
                // The error suggests that x doesnt live long enough.
                // This is what we call dangling reference.
    }
    // println!("{}", r);

    // Lets look at another example
    let str1 = String::from("Stringlejlwke");
    let str2 = String::from("Sejlwke");

    let result = longest(str1.as_str(), str2.as_str());
    print!("{}", result);
}

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len(){
//         x
//     }
//     else {
//         y
//     }
// }
//* As you can see in the above example, the borrow checker throws an error. 
// This is because of 2 reasons:
// 1. x and y can have varying lifetimes and their lifetimes arent fixed.
// 2. x and y can have different lifetimes from one another.

// !We can fix this issue by introducing Generic Lifetime Annotation.
// Generic lifetime annotation are used to describe the lifetimes of multiple
// references and how they relate to each other. 
//? &i32 -> a reference
//? &'a i32 -> a reference with an explicit lifetime
//? &'a mut i32 -> a mutable reference with an explicit lifetime

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len(){
        x
    }
    else {
        y
    }
}
//* In this scenario, the borrow checker will consider the lifetime of the 
//* reference that is generated from the function to be the smallest lifetime 
//* that is being passed in.

// !Lifetime Elisions
// 1. Each parameter that is a reference gets its own lifetime parameters
// 2. If there is exactly one input lifetime parameter, that lifetime is 
// assigned to all output lifetime parameters
// 3. If there are multiple input lifetime parameters, but one of them is 
// &self or &mut self the lifetime of self is assigned to all output lifetime
// parameters.