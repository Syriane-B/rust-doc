fn main() {
    // Variables
    println!("Hello, world!");
    let mut a = String::from("hello");
    print!("The value of a is: {}", a);
    a = String::from("test");
    print!("The value of a is: {}", a);

    let mut b: i32 = 5;
    println!("The value of b is: {}", b);
    b = 6;
    println!("The value of b is: {}", b);

    // Constants immutable
    // not allow to use mut obviously
    // cannot be string, only string slice
    const Y: i32 = 5;
    print!("The value of Y is: {}", Y);

    print!("The neighbours said : {}", OUTSIDE_SCOPE);
}

// It is possible to declare const outside scope
const OUTSIDE_SCOPE: &str = "Outside is fun";
