use std::fmt::Display;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    learn_struct();
    println!();

    learn_build_struct();
    println!();

    learn_struct_update();
    println!();

    learn_tuple_structs();
}

fn learn_struct() {
    println!("Learn Struct");

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };

    println!("{}", user1.email);

    user1.email = String::from("anotheremail@example.com");

    println!("{}", user1.email);
}

fn learn_build_struct() {
    println!("Learn Build Struct");

    let user = build_user(
        String::from("test@test.com"),
        String::from("test")
    );
    println!("{} {} {}", user.username, user.active, user.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

fn learn_struct_update() {
    println!("Learn Struct Update");

    let user1 = build_user(
        String::from("hello@hello.com"),
        String::from("hello")
    );

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("username123"),
        ..user1
    };

    println!("User 2 active: {}", user2.active);
}

fn learn_tuple_structs() {
    println!("Learn Tuple Structs");

    struct Color(i32, i32, i32);
    impl Display for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "({}, {}, {})", self.0, self.1, self.2)
        }
    }

    struct Point(i32, i32, i32);
    impl Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "({}, {}, {})", self.0, self.1, self.2)
        }
    }

    let black = Color(0, 0,0);
    let origin = Point(0, 0, 0);

    println!("black: {}", black);
    println!("origin: {}", origin);
}
