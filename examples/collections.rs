use std::collections::HashMap;

fn main() {
    let mut v = Vec::new();
    let s = String::from("hello");
    v.push(s); // Ownership moves to v
    // println!("{}", s); // Error: s was moved
    
    let mut map = HashMap::new();
    let key = String::from("numbers");
    let value = vec![1, 2, 3];
    map.insert(key, value); // map owns key and value
    println!("{:?}", map);
}