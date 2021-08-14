fn main() {
    example_one();

    println!();

    example_two();

    println!();

    return_tuple();

    println!();

    reference_borrowing();

    println!();

    slices_example();
}

fn example_one() {
    println!("Example One");

    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    // println!("{}", s);
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn example_two() {
    println!("Example Two");

    let s1 = gives_ownership();
    println!("{}", s1);

    let s2 = String::from("world");
    println!("{}", s2);

    let s3 = takes_and_gives_back(s2);
    println!("{}", s3);

    // println!("{}", s2);
}

fn gives_ownership() -> String {
    String::from("hello")
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn return_tuple() {
    println!("Return Tuple");

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn reference_borrowing() {
    println!("Reference Borrowing");

    let s1 = String::from("hello");

    let len = calculate_length_with_borrow(&s1);

    println!("The length of '{}' is {}", s1, len);

    let mut s2 = String::from("hello");

    change(&mut s2);

    println!("{}", s2);

    // let s3 = dangle();

    // println!("{}", s3);

    let s4 = no_dangle();

    println!("{}", s4);
}

fn calculate_length_with_borrow(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

fn slices_example() {
    println!("Slices Example");

    let mut s = String::from("hello world");

    let word = first_word(&s[..]);

    println!("The first word is '{}'", word);

    s.clear();

    println!("s is '{}'", s);

    other_slices();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn other_slices() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
