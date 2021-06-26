use std::io;

const OPTIONS: [&str; 3] = ["Fahrenheit to Celsius", "Celsius to Fahrenheit", "Exit"];

fn main() {
    loop {
        println!("Convert temperature:");
        for (index, option) in OPTIONS.iter().enumerate() {
            println!("\t{}. {}", index+1, option);
        }

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to readline");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice");
                continue;
            },
        };

        match choice {
            1 | 2 => (),
            3 => break,
            _ => {
                println!("Invalid choice");
                continue;
            }
        }

        conversion(choice);
    }
}

fn conversion(choice: u32) {
    loop {
        match choice {
            1 => println!("Fahrenheit: "),
            2 => println!("Celsius: "),
            _ => {
                println!("Invalid choice");
                break;
            }
        }

        let mut temperature_to_convert = String::new();

        io::stdin()
            .read_line(&mut temperature_to_convert)
            .expect("Failed to readline");

        let temperature_to_convert: f32 = match temperature_to_convert.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice");
                break;
            },
        };

        match choice {
            1 => {
                let temperature_converted = fahrenheit_to_celsius(temperature_to_convert);
                println!("Celsius (converted): {}", temperature_converted);
                break;
            },
            2 => {
                let temperature_converted = celsius_to_fahrenheit(temperature_to_convert);
                println!("Fahrenheit (converted): {}", temperature_converted);
                break;
            },
            _ => {
                println!("Invalid choice");
                break;
            }
        }
    }
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * (5.0/9.0)
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * (9.0/5.0)) + 32.0
}
