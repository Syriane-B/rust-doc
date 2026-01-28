fn main() {
    struct Book{
        title: String,
        author: String,
        pages: i32,
        available: bool,
    }

    struct User{
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // intit (can be mutable)
    let book = Book {
        title: String::from("My Title"),
        author: String::from("Anonymous"),
        pages: 10,
        available: true
    };

    let mut user1 = User {
        active: true,
        username: String::from("username1"),
        email: String::from("email1@gmail.com"),
        sign_in_count: 1,
    };

    user1.active = false;

    println!("User active: {}", user1.active);

    // return from function
    fn build_user(email: String, username: String) -> User {
        User{
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    let user2: User = User {
        email: String::from("email2@gmail.com"),
        ..user1
    };
    println!("User email: {}", user2.email);
    println!("User sign_in_count: {}", user2.username);

    // Tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    // Unit-Like struct
    struct AlwaysEqual;
    let subject = AlwaysEqual;

}
