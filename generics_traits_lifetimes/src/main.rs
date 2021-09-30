use std::fmt::{Display, Debug};

fn main() {
    println!("Without Generics");
    without_generics();
    println!();

    println!("With Generics");
    with_generics();
    println!();

    println!("Generic Struct");
    generic_struct();
    println!();

    println!("Traits");
    traits();
    println!();

    println!("Lifetimes");
    lifetimes();
    println!();

    println!("Struct Lifetimes");
    struct_lifetimes();
}

fn without_generics() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn with_generics() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_generic(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_generic(&char_list);
    println!("The largest char is {}", result);

    let number_list_2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let result = largest_generic_no_copy_or_clone(&number_list_2);
    println!("The largest number is {}", result);
}

fn largest_generic<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_generic_no_copy_or_clone<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    for item in list {
        if *item > *largest {
            largest = item
        }
    }

    largest
}

fn generic_struct() {
    let integer = Point { x: 5, y: 10 };
    println!("{:?}", integer);

    let float = Point { x: 1.0, y: 4.0 };
    println!("{:?}", float);

    let both_integer = PointMixed { x: 5, y: 10 };
    println!("{:?}", both_integer);

    let both_float = PointMixed { x: 1.0, y: 4.0 };
    println!("{:?}", both_float);

    let integer_and_float = PointMixed { x: 5, y: 4.0 };
    println!("{:?}", integer_and_float);

    let p1 = Point { x: 5, y: 10 };
    println!("p.x = {}", p1.x());

    let p2 = Point { x: 5.0, y: 10.0 };
    println!("distance_from_origin = {}", p2.distance_from_origin());

    let p3 = PointMixed { x: 5, y: 10.4 };
    let p4 = PointMixed { x: "Hello", y: 'c' };
    let p5 = p3.mixup(p4);
    println!("p5.x = {} , p5.y = {}", p5.x, p5.y);
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct PointMixed<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> PointMixed<T, U> {
    fn mixup<V, W>(self, other: PointMixed<V, W>) -> PointMixed<T, W> {
        PointMixed {
            x: self.x,
            y: other.y,
        }
    }
}

fn traits() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };
    println!("New article avaiable! {}", article.summarize());

    let pair = Pair::new(5, 10);
    pair.cmp_display();
}

pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.summarize_author(), self.location)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn _some_function<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    5
}

fn _returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people",),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self{ x, y }
    }
}

impl<T: PartialOrd + Display> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

fn lifetimes() {
    let x = 5;
    let r = &x;
    println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let string1 = String::from("test");
    let string2 = String::from("testtest");
    let result = longest_with_an_announcement(string1.as_str(), string2.as_str(), "I'm testing");
    println!("{}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }

    y
}

fn struct_lifetimes() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);

    println!("{}", i.level());

    println!("{}", i.announce_and_return_part("we're landing soon"));
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        return x;
    }
    y
}
