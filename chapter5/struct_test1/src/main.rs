#![allow(unused)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let mut user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("Hello {}", user.username);
    println!("email {}", user.email);

    user.email =  String::from("anotheremail@example.com");
    println!("change email {}", user.email);

    let user1 = User {
        email: String::from("someya@example.com"),
        username: String::from("test1"),
        ..user
    };

    println!("Hello {}", user1.username);
    println!("email {}", user1.email);
}
