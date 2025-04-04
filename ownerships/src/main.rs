fn main() {
    let s1 = String::from("Hello");
    takes_ownership(s1);
    let s2 = String::from("World");
    let x = 5;
    makes_copy(x);
    println!("{}", x);
    let s3 = gives_ownership();
    println!("{}", s3);
    let s4 = takes_and_gives_back(s3);
    println!("{}", s4);
    let len = calculate_length(&s4);
    println!("The length is {}.", len);
    let mut s5 = String::from("Hello");
    change(&mut s5);
    println!("{}", s5);
    let r1 = &s5;
    let r2 = &s5;
    println!("{} {}", r1, r2);
    let r3 = &mut s5;
    println!("{}", r3);
    // let reference_to_nothing = dangle();
    let mut m = String::from("Hello");
    let word = first_word(&m);
    println!("the first word is: {}", word);
    m.clear();
    let a = [1, 2, 3, 4, 5];
    let slice = &mut a.clone()[1..3];
    slice[1] = 10;
    println!("{:?}", slice);
    println!("{:?}", a);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("Hello");
//     &s
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
