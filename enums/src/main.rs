fn main() {
    println!("Example One");
    example_one();
    println!();

    println!("Example Two");
    example_two();
    println!();

    println!("Example Three");
    example_three();
    println!();

    println!("Example Four");
    example_four();
    println!();

    println!("Example Five");
    example_five();
    println!();

    println!("Options Example");
    options_example();
    println!();

    println!("Coins Example");
    coins_example();
    println!();

    println!("Option<T> Example");
    option_t_example();
    println!();

    println!("if let Example");
    if_let_example();
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn example_one() {
    let four = IpAddrKind::V4;
    println!("four: {:#?}", four);

    let six = IpAddrKind::V6;
    println!("six: {:#?}", six);
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn example_two() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("home: {:#?}", home);

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("loopback: {:#?}", loopback);
}

#[derive(Debug)]
enum IpAddrV2 {
    V4(String),
    V6(String),
}

fn example_three() {
    let home = IpAddrV2::V4(String::from("127.0.0.1"));
    println!("home: {:#?}", home);

    let loopback = IpAddrV2::V6(String::from("::1"));
    println!("loopback: {:#?}", loopback);
}

#[derive(Debug)]
enum IpAddrV3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn example_four() {
    let home = IpAddrV3::V4(127, 0, 0, 1);
    println!("home: {:#?}", home);

    let loopback = IpAddrV3::V6(String::from("::1"));
    println!("loopback: {:#?}", loopback);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        // TODO
    }
}

fn example_five() {
    let m1 = Message::Quit;
    m1.call();

    let m2 = Message::Move{
        x: 0,
        y: 0
    };
    println!("m2: {:#?}", m2);
    m2.call();

    let m3 = Message::Write(String::from("hello"));
    m3.call();

    let m4 = Message::ChangeColor(0, 0, 0);
    m4.call();
}

fn options_example() {
    let _x: i8 = 5;
    let _y: Option<i8> = Some(5);

    // let sum = x + y;
    // println!("Sum: {}", sum);
}

#[derive(Debug)]
enum UsState {
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn coins_example() {
    let _ = value_in_cents(Coin::Penny);
    let _ = value_in_cents(Coin::Nickel);
    let _ = value_in_cents(Coin::Dime);
    let _ = value_in_cents(Coin::Quarter(UsState::Alaska));

    let coin = Coin::Dime;

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1
    }
    println!("Non-quarter coins: {}", count);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn option_t_example() {
    let five = Some(5);
    let six = plus_one(five);
    assert_eq!(six, Some(6));
    let none = plus_one(None);
    assert_eq!(none, None);
}

fn if_let_example() {
    let some_u8_value = Some(0u8);

    // verbose
    match some_u8_value {
        Some(3) => println!("three"),
        _ => ()
    }

    // less verbose: if let
    if let Some(3) = some_u8_value {
        println!("three")
    }
}
