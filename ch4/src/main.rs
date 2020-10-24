fn main() {
    let s = String::from("hello");
    takes_ownership(s); // s is borrowed by fn, not available after here

    let x = 5;
    makes_copy(x);
    println!("{} x is still here", x);

    let s1 = String::from("Hello");
    let (s2, len) = takes_and_gives_back(s1);
    // here s1 is not available because it was taken by fn
    println!("s2: {}, length: {}", s2, len);

    let mut s3 = String::from("I am a long string");
    let s3_length = calculate_length(&s3);
    // s3 is still available because we did not transfer ownership
    println!("s3: {}, length: {}", s3, s3_length);

    change_string(&mut s3);
    let s3_length = calculate_length(&s3);
    println!("s3: {}, length: {}", s3, s3_length);

    // get the first word in a string
    let s4 = String::from("hello world");
    let word = first_word(&s4); // will get value 5 because it's the index of space
                                // s4.clear(); // empties the original string
    println!("The index of first space character : {}", word);
    println!("The first word in '{}' is '{}'", s4, &s4[0..word]);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}

fn makes_copy(some_integer: i32) {
    println!("Made copy {}", some_integer)
}

fn takes_and_gives_back(s: String) -> (String, usize) {
    // takes s and returns it
    let length = s.len();
    (s, length)
}

fn calculate_length(s: &String) -> usize {
    // this function does not take ownership of the string
    // it takes a reference of the string
    // note that Rust does not allow us to modify the value of s
    // because we only have the immutable reference to it
    s.len()
}

fn change_string(s: &mut String) {
    // this function changes the actual value of the string
    // because we have given the mutable reference to the original string
    s.push_str("And now I am longer");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
