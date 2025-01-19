
struct Blah {
    username : String,
    email : String,
    is_active : bool,
}

fn build_user(username:String, email:String, is_active:bool) -> Blah{
    let blah_2 =  Blah { username: username, email: email, is_active: is_active };
    blah_2
}

fn main() {
    let blah_1: Blah = Blah{
        username : String::from("Kekeke"),
        is_active : false,
        email: String::from("kekek@gmail.com")
    };
    let blah_3 = Blah{
        username : String::from("Kakakaka"),
        email: String::from("kakakak@gmail.com"),
        ..blah_1 // Gets whatever values were in blah_1. 
    };
    println!("{}, {}, {}", blah_1.email, blah_1.username, blah_1.is_active);
}
