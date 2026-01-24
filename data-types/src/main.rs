fn main() {
    // Int => i ou u (signed or not) 16 32 64 128
    let a: i16 = 32767;
    let b: i32 = 2147483647;
    let c: i64 = 9223372036854775807;
    println!("Signed Integer: {}, {}, {}", a, b, c);
    let d: u16 = 65535;
    let e: u32 = 4294967295;
    let f: u64 = 18446744073709551615;
    println!("Signed Integer: {}, {}, {}", d, e, f);

    // Floats f32 / f64
    let pi: f64 = 3.141592653589793;
    println!("Value od pi: {}", pi);

    //Boolean
    let is_snowing: bool = true;
    println!("Is it snowing ? {}", is_snowing);

    // Charachter Type  - char
    let letter: char = 'a';
    println!("letter is {}", letter);

    // Compound Data Types
    // arrays, tuples, slices ans strings (slice string)

    // Arrays
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array Length: {}", numbers.len());
    println!("Array: {:?}", numbers);
    // wrong example
    //let mix1: [i32; 5] = [5, 4, false];
    //let mix2: [i32; 5] = [5, 4, "apples"];
    // let mix2: [i32; 5] = [5, 4];
    // println!("Array: {:?}", mix);

    let fruits: [&str; 3] = ["Apple", "Banana", "Banana"];
    println!("Array: {:?}", fruits);
    println!("First string from array: {:?}", fruits[0]);

    // Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);

    //Slices: [1,2,3,4,5]
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number slice: {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal slices: {:?}", animal_slices);

    let book_slices :&[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"ZEN".to_string()];
    println!("Book slices: {:?}", book_slices);

    // String and slices
    let mut stone_cold: String = String::from("Hell, "); // can use String::new() without param if needed
    stone_cold.push_str("Yeah!");
    println!("Stone cold says: {}", stone_cold);

    let string: String = String::from("hello, world!");
    let slice: &str = &string[0..5];
    println!("slice values is: {}", slice);

}
