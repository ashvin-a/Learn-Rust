mod scalar_dtypes;
mod loops;

fn main() {
    
    let output = scalar_dtypes::scalar();
    println!("{:?}", output);

    let loops = loops::loops();
    println!("{:?}", loops);

    let mut x = 5;
    println!("The value of x is {}", x);
    
    x = 6;
    println!("New value of x : {}", x);

    let x :u32 = 7; // Shadowing. Preserves mutability. Can change types
    println!("The value of x is {}", x);
   
    let x = "6";
    println!("New value of x : {}", x);

    const SUB_COUNT: u32 = 100_000; // const cant be mut.
    // const can't be returned from a function. 
    // Numbers could have underscores

}
 