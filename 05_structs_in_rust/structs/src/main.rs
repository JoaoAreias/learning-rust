struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle{
    width: u32, 
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let user1 = User {
        email:  String::from("foo@bar.com"),
        username: String::from("john.smith"),
        active: true,
        sign_in_count: 9
    };

    let name = user1.username;

    let user2 = build_user(
        String::from("lorem@ipsum.com"), 
        String::from("lorem.ipsum")
    );

    struct Color(u8, u8, u8);
    struct Point(f64, f64, f64);

    let rect = Rectangle {
        height: 32, 
        width: 50
    };

    let rect2 = Rectangle {
        height: 10,
        width: 30
    };

    let rect3 = Rectangle::square(32);

    println!("rect: {:#?}", rect);
    println!("Area of the rectangle: {}", rect.area());
    println!("Can rect1 hold rect2? {}", rect.can_hold(&rect2));

}


fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 0, 
        active: true
    }
}