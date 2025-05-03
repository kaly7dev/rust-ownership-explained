fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s; // Immutable borrow
    let r2 = &s; // Another immutable borrow
    println!("{} {}", r1, r2); // Works fine
    
    let r3 = &mut s; // Mutable borrow
    r3.push_str(", world");
    println!("{}", r3);
}