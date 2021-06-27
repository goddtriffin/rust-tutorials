use std::io;

fn main() {
    loop {
        println!("Fibonacci numbers to compute:");

        let mut max_depth = String::new();

        io::stdin()
            .read_line(&mut max_depth)
            .expect("Failed to readline");

        let max_depth: i32 = match max_depth.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid");
                continue;
            },
        };

        if max_depth < 1 {
            println!("Must be greater than zero");
            continue;
        }

        let previous_depth: i32 = 2;
        let num1: u32 = 0;
        let num2: u32 = 1;
        let result = fibonacci(max_depth, previous_depth, num1, num2);
        println!("Result: {}", result);
    }
}

fn fibonacci(max_depth: i32, previous_depth: i32, num1: u32, num2: u32) -> u32 {
    let current_depth = previous_depth + 1;

    match max_depth {
        1 => {
            return num1;
        },
        2 => {
            return num2;
        },
        _ => ()
    }

    if current_depth == max_depth {
        return num1 + num2;
    }

    fibonacci(max_depth, current_depth, num2, num1 + num2)
}
