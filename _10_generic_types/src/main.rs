fn main() {
    let l1 = vec![1, 2, 3, 5, 6];
    println!("{}", get_largest_number(l1));

    let l2 = vec![57, 34, 2, 2, 65, 6];

    let l3 = vec!["a", "b", "ca", "asd"];
    println!("{}", get_largest(l3));
    println!("{}", get_largest(l2.clone()));
    println!("{}", get_largest_number(l2));

    let p1 = Point { x: "3", y: 32 };
    let p2 = Point {
        x: "qwe",
        y: "132s",
    };

    let p = Gen2 { x: 5, y: 4 };
    let p_with_float = Gen2 { x: 5.0, y: 4.0 };

    p.x(); //? Can only access x method.
    p_with_float.y(); //? This can access both x and y methods

    // Using the complex example
    let p1 = Blah { x: "3", y: 32 };
    let p2 = Blah {
        x: "qwe",
        y: "132s",
    };
    let p3 = p1.swap(p2);
    println!("{:?}", p3);
}

// Lets write a function that takes finds the largest out of a given list

fn get_largest_number(list: Vec<i32>) -> i32 {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//? What if we want to find the longest char??
// We do need to write another function with the same logic but different input and output type
// Instead we could use Generic types
//? Generic Type
fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    //? PartialOrd and Copy are the traits
    let mut largest = list[0];
    for item in list {
        if item > largest {
            // Should tell Rust that the input type will mostly be
            largest = item; // a type which is comparable. In Order to do that, we need
        } // !to mention the traits
    }
    largest
}

//? Using generics in Structs
// struct Point<T>{ // Here they are generic but both should be of the same type.
//     x: T,
//     y: T
// }

struct Point<T, U> {
    // This can have multiple types of attributes
    x: T,
    y: U,
}

struct Gen2<T> {
    // This can have multiple types of attributes
    x: T,
    y: T,
}
impl<U> Gen2<U> {
    fn x(&self) -> &U {
        &self.x
    }
}
impl Gen2<f64> {
    //? This implementation is only accessible if the parameter
    fn y(&self) -> f64 {
        //? is of type f64. Go check main function for implementation.
        self.x
    }
}
//? Option and Result enums use generic types
enum Option<T> {
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}

//* Another complex example */
#[derive(Debug)]
struct Blah<T, U> {
    x: T,
    y: U,
}

impl<T, U> Blah<T, U> {
    fn swap<V, W>(self, other: Blah<V, W>) -> Blah<T, W> {
        Blah {
            x: self.x,
            y: other.y,
        }
    }
}
