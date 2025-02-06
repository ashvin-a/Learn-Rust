use restaurant::eat_at_restaurant;

// use std::fmt::Result;
// use std::io::Result as IOResult;

use std::{fmt::Result, io::Result as IOResult}; // Same as above

fn main() {
    //? Binary Crates and Library Crates
    println!("Hello, world!");

    fn fun1() -> Result{
        Ok(())
    }

    fn fun2() -> IOResult<()>{ //* Using aliases */
        Ok(())
    }

    // For creating library file via cargo
    //? cargo run --lib project-name
    eat_at_restaurant()
}
