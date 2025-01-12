pub fn scalar() -> (char, &'static str, i32) {
    // integers, floats, bool, char
    // let x:u8 = 256; // Overflow occurs and throws error.
    // let x:f32 = 256.0;
    // let x:bool = true;
    let x: char = 't'; // Can only use single quotes

    let tup = ("Blahblah", 123, 'e');
    let (chan, sub_count, rand) = tup;
    println!("{}, {}, {}", chan, sub_count, rand);
    println!("{}, {}", tup.0, tup.1);

    // Arrays are fixed in length.
    let codes = [123, 3434, 433];
    println!("{}", codes[0]);

    // For conditions, it must be boolean. (if num{} does not work)

    // Loops
    let mut count = 0;
    let result = loop {
        if count == 10 {
            break count; //loops break here. And returns count.
        }
        println!("blah");
        count += 1;
    };
    println!("The count is : {}", result);
    return (x, chan, codes[2]);
}
