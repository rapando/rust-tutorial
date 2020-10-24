const PI: f32 = 3.14;

fn main() {
    let mut x = 5;
    println!("Value of x is {}", x);
    x = 6;
    println!("Value of x is {}", x);
    println!("Pi = {}", PI);

    let a: u8 = 255;
    println!("a : {}", a);

    let tup: (u32, f64, u8, char) = (500, 6.4, 1, 'A');
    let (b, c, d, e) = tup;
    println!("b = {}, c = {}, d = {}, e = {}", b, c, d, e);

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array first element: {}", arr[0]);

    let double34: u32 = another_function(34);
    println!("Double of 34 : {}", double34);

    find_greater(34, 64);

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value at {} is {}", index, a[index]);
        index += 1
    }

    for element in a.iter() {
        println!("Element : {}", element);
    }
}

fn another_function(a: u32) -> u32 {
    println!("This is another function with a : {}", a);
    a * 2
}

fn find_greater(a: i32, b: i32) {
    if a < b {
        println!("A is less than B");
    } else if b < a {
        println!("A is greater than B");
    } else {
        println!("They are equal!");
    }
}
