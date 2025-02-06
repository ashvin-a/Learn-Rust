pub mod hosting;

pub struct Breakfast {
    pub toast: String, // ! Everything in Rust is private by default.
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}
pub enum Appetizer {
    //* If you mark an Enum public, the attributes will also be public
    Soup, //* This is not the case for implementations.
    Salad,
}
mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
}
