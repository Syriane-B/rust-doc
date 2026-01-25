fn main() {
    //ownership
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    //s2 now has the ownership
    let _s2 = s1;

    // no longer exists
    // println!("The length of '{}'.", s1);

    //References and Borrowing => here is the main concept of Rust to avoid errors (null pointer errors in C)
    // Immutable Reference
    // Mutable Reference
    // Create a reference using &
    let mut x: i32 = 5;
    let _r: &mut i32 = &mut x;
    *_r += 1;
    *_r -= 3;
    println!("The value of x: '{}'.", x);
    // cannot borrow as immutable because already borrowed as mutable
    //println!("The value of _r: '{}'.", _r);

}

// ownership
fn calculate_length(s: &String) -> usize {
    s.len()
}
//References and Borrowing
