fn main() {
    // the difference between mutable variable and shadowing
    // cannot assign a new value to the variable like x = 10;
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x l.7 is : {}", x);
    }
    println!("The value of x l.9 is : {}", x);

    // can change type of a variable
    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of space is : {}", spaces);


}
