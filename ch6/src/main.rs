/*
 * Enums and Pattern Matching
*/

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home      : {:?}", home);
    println!("Loopback  : {:?}", loopback);

    let some_u8_value = 2u8;
    match some_u8_value {
        1 => println!("One"),
        2 => println!("Two"),
        _ => (),
    };

    println!("");

    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    };

    if let Some(3) = some_u8_value {
        println!("three");
    }
}
