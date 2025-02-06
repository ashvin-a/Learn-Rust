//? Wew new stuffs! Created via `cargo new --lib library-folder`
mod frontend; // * Importing modules be like. It has to have the same file name.

pub use crate::frontend::hosting;

pub fn eat_at_restaurant() {
    // * Relative path
    frontend::hosting::add_to_waitlist();

    use frontend::hosting;
    hosting::add_to_waitlist(); // Same as above
                                // * Absolute path
                                // crate::frontend::hosting::add_to_waitlist();

    //? In Rust, Parent class can't see whats happening in child class
    //? whereas the child class can see the Parent class

    let mut meal = frontend::Breakfast::summer("Blah");
    meal.toast = String::from("Bleh"); // Inorder to reassign a parameter, it has to be public.

    let order1 = frontend::Appetizer::Salad;
    let order2 = frontend::Appetizer::Soup;
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
