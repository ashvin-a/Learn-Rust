use core::error;
use std::{fmt::Error, fs::{read_to_string, File}, io::{self, BufRead, BufReader, ErrorKind, Read}};

fn main() {
    // a();
    // How to handle error if it occurs
    let f = File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound{
            File::create("hello.txt").unwrap_or_else( |error|{
                panic!("Error in creating file! {:?}", error);
            })
        }
        else{
            panic!("Error in opening the file! {:?}", error)
        }
    }
    );
    // match read_to_string("hello.txt"){
    //     Ok(contents) => println!("The file content : {:?}", contents),
    //     Err(error)=> println!("An error occured : {:?}", error)
    // }
    //? OR we could do something like this
    let reader = BufReader::new(f); //? By doing so we can use f.
    println!("The file content :");
    for line in reader.lines() {
        match line {
            Ok(content) => println!("{:?}", content),
            Err(error) => println!("Error reading line: {:?}", error),
        }
    }

    fn read_username_from_file() -> Result<String, io::Error>{
        let mut s = String::new();
        //? The `?` operator is used when we need the error to be thrown by
        //? the callable function (if an error occure that is..)
        File::open("hello.txt")?.read_to_string(&mut s)?; 
        Ok(s)
    }
    println!("{:?}",read_username_from_file());
}

fn a(){
    b();
}

fn b(){
    panic!("An error happened here");
}



