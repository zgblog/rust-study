struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    fn square(size: u32) -> Rectangle { // 关联方法
        Rectangle {
            width: size,
            length: size
        }
    }
}

struct Color(i32, i32, i32);    // tuple struct

fn main() {
    println!("Hello, world!");

    let user1 = User {
        username: String::from("zgblog"),
        email: String::from("zhougang6633@gmail.com"),
        sign_in_count: 123,
        active: true    
    };

    let w = 30;
    let l = 50;

    let rect = Rectangle {
        width: 30,
        length: 50
    };

    println!("{}", rect.area());

    println!("{:#?}", rect);

    let a = Rectangle::square(32);

    
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 123
    }
}

fn update(user: User) -> User {
    User {
        active: false,
        sign_in_count: 333,
        ..user
    }
}

fn area(width: u32, length: u32) -> u32 {
    width * length
}

fn area2(dim: &Rectangle) -> u32 {
    dim.width * dim.length
}

