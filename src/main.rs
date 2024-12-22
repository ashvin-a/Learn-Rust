use learn_rust::greet;

fn main() {
    greet();
    let width = 4;
    let height = 7;
    let depth = 10;

    {
        let area = area_of(width, height);
        // println!("Area is {}", area);
    }
         println!("Volume is {}", volume(width, height, depth));
}
fn area_of(_x: i32, _y: i32) -> i32 {
    // 2a. Fix this function to correctly compute the area of a rectangle given
    // dimensions x and y by multiplying x and y and returning the result.
    //
    return _x*_y;
    // Challenge: It isn't idiomatic (the normal way a Rust programmer would do things) to use
    //            `return` on the last line of a function. Change the last line to be a
    //            "tail expression" that returns a value without using `return`.
    //            Hint: `cargo clippy` will warn you about this exact thing.
}

fn volume(h:i32, l:i32, b:i32) -> i32 {
    return h*l*b;
}