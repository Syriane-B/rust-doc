fn main() {
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    //s2 now hace the ownership
    let s2 = s1;

    // no longer exists
    // println!("The length of '{}'.", s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
// Ownership
