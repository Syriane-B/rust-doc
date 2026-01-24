// variables declared outside functions must be declared as const and in UPPERCASE_SNAKE_CASE
const HOURS_IN_ON_DAY: i32 = 24;

fn main() {
    println!("For sure there is {}h in one day", HOURS_IN_ON_DAY);
    tell_height(167);
    human_id("Joel", 30, 168.5);

    let x= {
        let price: i32 = 5;
        let qty: i32= 10;
        // this last line without ; mean that x value will use the value of the math operation (last line)
        // no need to return
        price * qty
    };

    println!("Result is: {}", x);

    println!("add 3 an 4, result is {}", add(3, 4));
    let res: i32 = add(5,10);

    println!("Saved result is: {}", res);

    // calling BMI function
    let weight: f64 = 70.0;
    let height: f64= 1.80;

    let bmi: f64 = calculate_bmi(weight, height);
    println!("BMI is: {}", bmi);
}

fn tell_height(height: u32){
    println!("My height is {} cm.", height);
}

fn human_id(name: &str, age:u32, height: f32) {
    println!("My name is {}, I am {} years old and my height is {} cm.", name, age, height);
}

// function returning value
fn add(a: i32, b: i32) -> i32 {
    a + b
}
// Expressions and statement
// Expression: returns a value (5 | true & false | add(3,4) | {code}
//
// Statement Does not return value (as void in php)
// Almost all statement ends with ;
// examle variable declaration let x = 5;
// function def fn foo() {};
// if condition {value1} else {value2}

// example BMI = weight(kg)/height(m)^2
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}
