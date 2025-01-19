struct Blah {
    username: String, // Could have referencing data(string slices,etc.)
    // But that requires LIFETIME.
    email: String,
    is_active: bool,
}

#[derive(Debug)] // Supports printing the object if this class and other stuffs.
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Implementing methods.

    fn calculate_area(&self) -> u32 {
        // Stores the instance. Usually a reference to the
        // instance but could also be owned.
        self.height * self.width
    }

    fn compare(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.height
    }

    fn square(size: u32) -> Rectangle {
        // Associative function. So no self parameter.
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn build_user(username: String, email: String, is_active: bool) -> Blah {
    let blah_2 = Blah {
        username: username,
        email: email,
        is_active: is_active,
    };
    blah_2
}

fn calculate_area(height: u32, width: u32) -> u32 {
    height * width
}

// Using tuples
fn calculate_area_new(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn calculate_area_obj(dim: &Rectangle) -> u32 {
    // We dont wanna take the ownership of the Class.
    dim.height * dim.width
}

fn main() {
    let blah_1: Blah = Blah {
        username: String::from("Kekeke"),
        is_active: false,
        email: String::from("kekek@gmail.com"),
    };
    let blah_3 = Blah {
        username: String::from("Kakakaka"),
        email: String::from("kakakak@gmail.com"),
        ..blah_1 // Gets whatever values were in blah_1.
    };

    let rect1 = Rectangle {
        width: 32,
        height: 23,
    };

    let rect2 = Rectangle {
        width: 23,
        height: 13,
    };

    println!(
        "{}, {}, {}",
        blah_1.email, blah_1.username, blah_1.is_active
    );
    println!("{:?}", calculate_area(2, 3));

    let dim = (3, 4);
    println!("{:?}", calculate_area_new(dim));

    println!("{:?}", calculate_area_obj(&rect1));

    println!("{:#?}", rect1); // # For prettier formatting.

    println!("{:#?}", rect1.calculate_area());

    println!("{:#?}", rect1.compare(&rect2));

    println!("{:#?}", Rectangle::square(43)); // Invoking an associative function
}
