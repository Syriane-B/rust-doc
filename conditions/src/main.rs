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

    // loops
    // Loop unconditional
    // loop {
    //     println!("Hello World");
    // }

    let mut counter = 0;
    let result: u16 = loop {
        counter += 1;
        if counter == 10 {
            break counter +100;
            // error if break counter -100
        }
    };
    println!("The result is {result}");


    // possible to break parent loop. using a loop name prefixed with 'loop_name
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    // While
    let mut counter = 0;
    while counter <= 3 {
        println!("count = {}", counter);
        counter += 1;
    }
    println!("Hello");

    // for elements

    let a = [1, 2, 3, 4, 5];
    let b = ["a", "b", "c"];
    for element in a {
        for letter in b {
            println!("Matix: {element}|{letter}");
        }
    }

}
