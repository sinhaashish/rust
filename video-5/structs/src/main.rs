fn main() {
    println!("Hello, world!");
    let user1 = User {
        email: String::from("someone   @example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("The value of user1 is: {}", user1.email);
    let user2 = build_user(String::from("someone   @example.com"), String::from("someusername123"));
    println!("The value of user2 is: {}", user2.email);
    let user3 = User {
        email: String::from("someone   @example.com"),
        username: String::from
        ("  someusername123"),
        ..user1
    };
    println!("The value of user3 is: {}", user3.username);
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is: {}", rect1.area());
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    let sq = Rectangle::square(3);
    println!("The area of the square is: {}", sq.area());

}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

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
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
