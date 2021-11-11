use core::slice;
use std::{fmt, ops::Add};

static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    println!("Unsafe Rust");
    unsafe_rust();
    println!();

    println!("Advanced Traits");
    advanced_traits();
    println!();

    println!("Advanced Types");
    advanced_types();
    println!();

    println!("Advanced Functions and Closures");
    advances_functions_and_closures();
    println!();

    println!("Macros");
    macros();
}

fn unsafe_rust() {
    // dereferencing a raw pointer
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // let address = 0x012345usize;
    // let _r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // calling an unsafe function or method
    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }

    // creating a safe abstraction over unsafe code
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = split_at_mut(r, 3);
    assert_eq!(a, &mut[1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    println!("first: {:?}, second: {:?}", a, b);

    // using extern functions to call external code
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // accessing or modifying a mutable static variable
    println!("name is: {}", HELLO_WORLD);

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // implementing an unsafe trait
    unsafe trait Foo {
        // does something
    }
    unsafe impl Foo for i32 {
        // do the thing
    }
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut[i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn advanced_traits() {
    // default generic type parameters and operator overloading
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    impl Add for Point {
        type Output = Point;

        fn add(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    assert_eq!(
        Point{ x: 1, y: 0 } + Point{ x: 2, y: 3 },
        Point{ x: 3, y: 3 }
    );

    #[derive(Debug, PartialEq)]
    struct Millimeters(u32);
    struct Meters(u32);
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, rhs: Meters) -> Self::Output {
            Millimeters(self.0 + (rhs.0 * 1000))
        }
    }
    assert_eq!(
        Millimeters(50) + Meters(2),
        Millimeters(2050)
    );

    // fully qualifying syntax for disambiguation: calling methods with the same name
    trait Pilot {
        fn fly(&self);
    }
    trait Wizard {
        fn fly(&self);
    }
    struct Human;
    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }
    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }
    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);

    trait Animal {
        fn baby_name() -> String;
    }
    struct Dog;
    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }
    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // using supertraits to require one trait's functionality within another trait
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
    struct NewPoint {
        x: i32,
        y: i32,
    }
    impl fmt::Display for NewPoint {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    impl OutlinePrint for NewPoint {}
    let p = NewPoint {
        x: 5,
        y: 10,
    };
    p.outline_print();

    // using the newtype pattern to implement external traits on external types
    struct Wrapper(Vec<String>);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

fn advanced_types() {
    // types synonyms with type aliases
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>;
    let _f: Thunk = Box::new(|| println!("hi"));
    fn _takes_long_type(_f: Thunk) {
        // take a thunk
    }
    // fn returns_long_type() -> Thunk {
    //     // return a thunk
    // }

    // dynamically sized types and the sized trait
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";
    fn _generic<T>(_t: T) {
        // do something generic
    }
}

fn advances_functions_and_closures() {
    // function pointers
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("list_of_strings = {:?}", list_of_strings);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("list_of_strings = {:?}", list_of_strings);

    #[derive(Debug)]
    enum Status {
        Value(u32),
        _Stop,
    }
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("list_of_statuses = {:?}", list_of_statuses);

    // returning closures
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
    println!("returned closures returns: {}", returns_closure()(5))
}

#[macro_export]
macro_rules! myvec {
    ( $( $x:expr),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn macros() {
    // declarative macros
    let v: Vec<u32> = myvec!(1, 2, 3);
    println!("myvec = {:?}", v);
}
