use std::collections::HashMap;
// Hashmaps in rust are a collection of key value pairs

fn main() {
    let mut dict = HashMap::<i8, i8>::new();

    dict.insert(1, 2); // Insert key value pair
    dict.insert(2, 3);

    println!("{:?}", dict);
    println!("1: {:?}", dict.get((&1i8))); // Get value by key
}
