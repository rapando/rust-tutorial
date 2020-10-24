/*
 *
 * Common Collections
 * Vector   : store a variable number of values next to each other
 * String   : Collection of characters
 * Hash Map : You know what this is
 *
*/
use std::collections::HashMap;

fn main() {
    // let v = vec![1, 2, 3]; // Rust infers the data type

    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third = &v[2];

    println!("The third element in vector = {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    };

    for i in &v {
        println!("{}", i);
    }

    // string
    let s1 = String::from("Hello, ");
    let s2 = "World".to_string();
    let s3 = s1 + &s2;
    println!("s1 has been moved.\ns2={}, s3={}", s2, s3);
    let s3 = format!("{} - {}", s2, s3); // format! does not take ownership
    println!("s1 has been moved.\ns2={}, s3={}", s2, s3);

    // hash maps
    // one way of constructing a hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // the other
    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];
    // let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // read
    // to update a hash map, use insert
    // let key = String::from("Blue");
    // let score = scores.get(&key);

    scores.insert(String::from("Blue"), -32);
    for (key, value) in &scores {
        println!("{}:{}", key, value);
    }
    println!("{:?}", scores);

    // only insert if the key has no value
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Green")).or_insert(50);
    println!("{:?}", scores);

    // update an value based on the old value
    let text = "hello world hello world";
    let mut map = HashMap::new();
    for word in text.split_ascii_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);


}
