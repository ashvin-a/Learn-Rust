fn main() {
    // Collections include vectors, hashmaps, and strings
    let a = [1, 2, 3];

    let mut vec: Vec<i32> = Vec::new();
    vec.push(3);
    vec.push(54);
    vec.push(6);
    vec.push(93);
    println!("{:?}, {:?}", vec, a);

    {
        // ! Collections are all stored as heap. Thus, when the
        // ! scope of the variable is over, it gets deallocated.
        let mut v2 = vec![1, 2, 3];
        let v3 = vec![1, 2, 3];

        // Accessing elements
        let elem = &v2[1];
        println!("{:?}", elem);

        //? In arrays, out of bound error occurs during compile time itself since we know the length of the array.
        //? But in case of vectors, since its stored in heap, it had variable length. Thus crashes during runtime.

        // A solution for this. Handles out of bound exceltion in vectors
        let elem2 = match v2.get(2) {
            Some(elem) => String::from(elem.to_string()),
            None => String::from("No elements found, bitch!"),
        };
        println!("{:?}", elem2);

        //? Iterating through vectors

        for i in v3 {
            println!("{:?}", i)
        }
        // Altering the values inside a vector
        for i in &mut v2 {
            // Taing a mutable reference
            *i += 10 // Dereference operator -> To get the underlying value and make change to it.
        }
        println!("{:?}", v2)
    }
}
