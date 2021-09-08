use std::{collections::HashMap, fmt};

fn main() {
    println!("Vectors");
    vectors();
    println!();

    println!("Strings");
    strings();
    println!();

    println!("Hash Maps");
    hashmaps();
}

fn vectors() {
    let _v: Vec<i32> = Vec::new();
    let _w = vec![1, 2, 3];

    let mut x = Vec::new();
    x.push(5);
    x.push(6);
    x.push(7);
    x.push(8);

    let third: &i32 = &x[2];
    println!("The third element is {}", third);

    match x.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut y = vec![100, 32, 57];
    for i in &y {
        println!("{}", i);
    }

    for i in &mut y {
        *i += 50;
    }

    for i in &y {
        println!("{}", i);
    }

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    impl fmt::Display for SpreadSheetCell {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                SpreadSheetCell::Int(int) => write!(f, "{}", int),
                SpreadSheetCell::Float(float) => write!(f, "{}", float),
                SpreadSheetCell::Text(text) => write!(f, "{}", text),
            }
        }
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];

    for i in &row {
        println!("{}", i);
    }
}

fn strings() {
    let mut s = String::new();
    println!("s is {}", s);

    let data = "initial contents";
    s = data.to_string();
    println!("s is {}", s);

    s = String::from("foo");
    s.push_str("bar");
    println!("s is {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut t = String::from("lo");
    t.push('l');
    println!("t is {}", t);

    let u1 = String::from("Hello, ");
    let u2 = String::from("world!");
    let u3 = u1 + &u2;
    println!("{}", u3);

    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");
    let t5 = format!("{}-{}-{}", t1, t2, t3);
    println!("{}", t5);
    let t4 = t1 + "-" + &t2 + "-" + &t3;
    println!("{}", t4);

    let v = "नमस्ते".to_string();
    for c in v.chars() {
        print!("{} ", c);
    }
    println!();
    for b in v.bytes() {
        print!("{} ", b);
    }
    println!();
}

fn hashmaps() {
    let mut scores1 = HashMap::new();
    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Yellow"), 50);
    println!("{:?}", scores1);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores2);

    let field_name = String::from("Favourite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{:?}", map);

    let team_name = String::from("Blue");
    match scores1.get(&team_name) {
        None => println!("Score: No score for team {}", &team_name),
        Some(score) => println!("Score: {}", score),
    }

    for (key, value) in &scores1 {
        println!("{}: {}", key, value);
    }

    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Blue"), 10);
    scores3.insert(String::from("Blue"), 25);
    println!("{:?}", scores3);

    let mut scores4 = HashMap::new();
    scores4.insert(String::from("Blue"), 10);
    scores4.entry(String::from("Yellow")).or_insert(50);
    scores4.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores4);

    let text = "hello world wonderful world";
    let mut map2 = HashMap::new();
    for word in text.split_whitespace() {
        let count = map2.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map2);
}
