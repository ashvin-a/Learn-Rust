//? Wew new stuffs! Created via `cargo new --lib library-folder`
mod frontend {
    pub mod hosting {
        pub fn add_to_waitlist(){

        }

        fn seat_handling(){

        }
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

pub fn eat_at_restaurant(){
    // * Relative path
    frontend::hosting::add_to_waitlist();

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
