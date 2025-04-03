fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = "six";
    println!("The value of x is: {}", x);

    const SUBSCRIBER_COUNT: u32 = 100_000;
    println!("The value of SUBSCRIBER_COUNT is: {}", SUBSCRIBER_COUNT);

    let a = 98_200;
    let b = 0xff;
    let c = 0b1111_0000;
    let d = b'A';
    let f: u8 = 255;

    let tup = ("Let's Get Rusty", 100_000);
    let (channel_name, subscriber_count) = tup;
    let sub_count = tup.1;
    let error_codes = [404, 403, 401];
    let not_found = error_codes[0];
    let byte = [0; 8];
    let sum = my_function(not_found, sub_count);
    println!("The value of sum is: {}", sum);
    let number = if (not_found > error_codes[1]) { 5 } else { 6 };
    println!("The value of number is: {}", number);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The value of result is: {}", result);

    let arr = [1, 2, 3, 4, 5];
    for element in arr.iter() {
        println!("The value of element is: {}", element);
    }

    for number in 1..4 {
        println!("The value of number is: {}", number);
    }
}
fn my_function(x: i32, y: i32) -> i32 {
    x + y
}
