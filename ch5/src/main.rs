#[derive(Debug)] // for printing the whole struct at once

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // define a method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // association functions don't use self
    // often used for constructors that will return a new instance of the struct
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let email = String::from("rapando@mail.com");
    let username = String::from("rapando");

    let user1 = build_user(email, username);
    let user2 = User {
        email: String::from("max@mail.com"),
        username: String::from("max"),
        ..user1
    };
    println!(
        "Hello, {} {}, {}, {}",
        user1.username, user1.email, user1.active, user1.sign_in_count
    );

    println!("Printing the whole struct {:?}", user2);

    // using methods
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("The rectangle dimensions : {:?}", rect1);
    println!("The area of the rectangle is {}", rect1.area());

    // to use an associative function we use ::
    let sq = Rectangle::square(3);
    println!("The square dimensions : {:?}", sq);
    println!("The area of the square : {}", sq.area());
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}
