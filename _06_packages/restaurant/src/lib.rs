//? Wew new stuffs! Created via `cargo new --lib library-folder`
mod frontend {
    pub mod hosting {
        pub fn add_to_waitlist(){

        }

        fn seat_handling(){

        }
    }

    pub struct Breakfast{
        pub toast: String, // ! Everything in Rust is private by default.
        seasonal_fruit: String
    }

    impl Breakfast{
        pub fn summer(toast: &str)-> Breakfast{
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
    pub enum Appetizer{ //* If you mark an Enum public, the attributes will also be public
        Soup,           //* This is not the case for implementations.
        Salad
    }
    mod serving{
        fn take_order(){

        }
        fn serve_order(){
 
        }
        fn take_payment(){

        }

    }
}
pub use crate::frontend::hosting;

pub fn eat_at_restaurant(){
    // * Relative path
    frontend::hosting::add_to_waitlist();

    use frontend::hosting;
    hosting::add_to_waitlist(); // Same as above
    // * Absolute path
    // crate::frontend::hosting::add_to_waitlist();

    //? In Rust, Parent class can't see whats happening in child class
    //? whereas the child class can see the Parent class
}




// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
