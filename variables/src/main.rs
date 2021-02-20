fn main() {
    let mut a = 5;
    println!("The value of x is: {}", a);
    a = 6;
    println!("The value of x is: {}", a);

    let b = 5;
    let b = b + 1;
    let b = b * 2;
    println!("The value of y is: {}", b);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    let guess: i32 = "42".parse().expect("Not a number!");
    println!("The values of guess is: {}", guess);

    let c = 2.0;
    println!("The values of c is: {}", c);
    let d: f32 = 3.0;
    println!("The values of d is: {}", d);

    let sum = 5 + 10;
    println!("The values of sum is: {}", sum);
    let difference = 95.5 - 4.3;
    println!("The values of difference is: {}", difference);
    let product = 4 * 30;
    println!("The values of product is: {}", product);
    let quotient = 56.7 / 32.2;
    println!("The values of quotient is: {}", quotient);
    let remainder = 43 % 5;
    println!("The values of remainder is: {}", remainder);

    let e = true;
    println!("The value of e is: {}", e);
    let f: bool = false;
    println!("The value of f is: {}", f);

    let g = 'z';
    println!("The value of g is: {}", g);
    let h = 'ℤ';
    println!("The value of h is: {}", h);
    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tuple is: ({}, {}, {})", tuple.0, tuple.1, tuple.2);
}
