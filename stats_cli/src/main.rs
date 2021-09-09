use std::{collections::HashMap, io};

fn main() {
    loop {
        println!("Stats CLI");
        println!("==========");
        println!("Enter any number of space-delimited integers: ");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to readline\n");

        let mut nums: Vec<i32> = choice.trim().split(" ").filter_map(|x| x.parse::<i32>().ok()).collect();
        nums.sort();

        if nums.len() == 0 {
            println!("No numbers found\n");
            continue;
        }

        println!("Average: {}", average(&nums));
        println!("Median: {}", median(&nums));
        println!("Mode: {}", mode(&nums));
        println!();
    }
}

fn average(nums: &Vec<i32>) -> f32 {
    let sum: i32 = nums.iter().sum();
    let count: f32 = nums.len() as f32;
    (sum as f32) / count
}

fn median(nums: &Vec<i32>) -> i32 {
    match nums.get(nums.len() / 2) {
        Some(num) => *num,
        None => -469,
    }
}

fn mode(nums: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for num in nums {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut highest_key = &i32::MIN;
    let mut highest_value = &i32::MIN;
    for (key, value) in &map {
        if *value > *highest_value {
            highest_key = &key;
            highest_value = &value;
        }
    }
    *highest_key
}
