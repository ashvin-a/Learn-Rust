fn main() {

let l1 = vec![1,2,3,5,6];
println!("{}",get_largest_number(l1));

let l2 = vec![57,34,2,2,65,6];

let l3 = vec!["a", "b", "ca", "asd"];
println!("{}", get_largest(l3));
println!("{}", get_largest(l2.clone()));
println!("{}",get_largest_number(l2));

let p1 = Point{x:"3", y:32};
let p2 = Point{x:"qwe", y:"132s"};

}

// Lets write a function that takes finds the largest out of a given list

fn get_largest_number(list: Vec<i32>) -> i32{
    let mut largest = list[0];
    for item in list{
        if item > largest{
            largest = item;
        }
    }
    largest
}

//? What if we want to find the longest char??
// We do need to write another function with the same logic but different input and output type
// Instead we could use Generic types
//? Generic Type
fn get_largest<T : PartialOrd + Copy>(list:Vec<T>) -> T{ //? PartialOrd and Copy are the traits
    let mut largest = list[0];
    for item in list{
        if item > largest{ // Should tell Rust that the input type will mostly be 
            largest = item;// a type which is comparable. In Order to do that, we need
        }                  // !to mention the traits 
    }
    largest
}

//? Using generics in Structs
// struct Point<T>{ // Here they are generic but both should be of the same type.
//     x: T,
//     y: T
// }

struct Point<T, U>{ // This can have multiple types of attributes
    x: T,
    y: U
}

//? Option and Result enums use generic types
enum Option<T>{
    Some(T),
    None
}
enum Result<T, E>{
    Ok(T),
    Err(E),
}