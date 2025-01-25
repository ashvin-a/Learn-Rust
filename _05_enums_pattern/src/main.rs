
enum IPAddressKind {
    V4(String), // Helps store data inside variants
    V6(String),
}

struct IPAddress {
    kind: IPAddressKind,
    ip_address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn _some_function() {
        // This is how we pass method into enums.
    }
}

// !There is no variable in Rust that is Null.
// In such scenarios, we use enum Option<T>
// enum Option<T>{
//     Some(T),
//     None
// }

// ?Another use case of Enums
enum Coins {
    Dime,
    Rupee(State),
    Pound,
}

#[derive(Debug)]
enum State {
    Kerala,
    Karnataka,
    Goa,
    Delhi,
}

fn value_of_coin(coin: Coins) -> i32 {
    match coin {
        Coins::Dime => 1,
        // Coins::Rupee => 10,
        Coins::Rupee(state) => {
            println!("{:?}", state);
            10
        }
        Coins::Pound => 20,
    }
}

fn main() {
    let _four = IPAddressKind::V4;
    let _six = IPAddressKind::V6;

    // let localhost = IPAddress {
    //     kind: IPAddressKind::V4,
    //     ip_address: String::from("127.0.0.1"),
    // };
    let _localhost = IPAddressKind::V4(String::from("127.0.0.1")); // Same thing as
                                                                   // above but more concise.

    // Some variables with None.
    let _some_number = Some(5); // This means it could be an integer or it could be Null.
    let _some_string = Some("Blah blah");

    let _absent_number: Option<i32> = None; // See! None -> Option<>

    let x = 5;
    let y: Option<i32> = Some(5);

    println!("Sum : {:?}", x + y.unwrap_or(0)); // ?Takes the value of y. If y is None, default value is used.

    println!(
        "{:#?}, {:#?}, {:#?}",
        value_of_coin(Coins::Dime),
        value_of_coin(Coins::Rupee(State::Kerala)),
        value_of_coin(Coins::Pound)
    );

    let plus_var = None;
    let var = Some(5);
    println!("{:?}", plus_one(plus_var));
    println!("{:?}", plus_one(Some(32)));
    println!("{:?}", plus_one(var));

    let some_value = Some(3);
    match some_value {
        Some(3) => println!("three"),
        _ => (), // Does nothing if any other value is passed into some_value
    }

    //? If let syntax - Quite confusing.
    if let Some(3) = some_value {
        println!("HEHEE its Some(3)")
    };
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // None => None, Commented this out because the last condition showed warning of unreachable pattern
        Some(i) => Some(i + 1),
        _ => None, // This says that if x is of any other pattern, return None
    }
}
