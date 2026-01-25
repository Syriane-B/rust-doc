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

    // stuct
    let mut alice_account: BankAccount = BankAccount{
        owner: String::from("Alice"), // "Alice".to_string()
        balance: 155.55,
    };
    // immutable borrow to check the balance
    alice_account.check_balance();

    // mutable borrow
    alice_account.withdraw(45.5);

    alice_account.check_balance();

}

// ownership
fn calculate_length(s: &String) -> usize {
    s.len()
}
//References and Borrowing

// struct this is the properties of a Class
// just for example, should never use float, use of rust_decimal crate
struct BankAccount {
    owner: String,
    balance: f64,
}

// impl this is where all methods are defined
impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdraw {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("The balance of '{}' is {}.", self.owner, self.balance);
    }
}