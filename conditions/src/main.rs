fn main() {
    // Control flow
    // Conditions if else
    let age: u16 = 15;
    if age >= 18 {
        println!("you can drive a car");
    } else {
        println!("you cannot drive a car");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    }

    let condition = false;
    // defining a value depending a condition result
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);


}
