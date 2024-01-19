use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io;

fn main() {
    let mut file_name = String::new();
    println!("Enter your file name:");

    let _n = io::stdin()
        .read_line(&mut file_name)
        .expect("Nothing entered, sir.");
    file_name = file_name.trim().to_string();

    let contents: String = fs::read_to_string(file_name).expect("Failed to read file");

    let answer = part1(&contents);
    println! {"Answer: {answer}"}
}

fn part1(input: &str) -> i64 {
    let mut sum: i64 = 0;

    for line in input.lines() {
        println!("{line}");
        let matches: Vec<&str> = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine")
            .unwrap()
            .find_iter(line)
            .map(|m| m.as_str())
            .collect();
        let first_match = matches[0];
        let second_match: &str = matches[matches.len() - 1];

        let combined_string = format!(
            "{}{}",
            convert_str_digit(first_match),
            convert_str_digit(second_match)
        );
        println!("Adding {}", combined_string);
        sum += combined_string
            .parse::<i64>()
            .expect("Not a digit, my good lad.");
    }
    sum
}

fn convert_str_digit(input: &str) -> &str {
    let text_to_digit: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    match text_to_digit.get(input) {
        Some(x) => *x,
        _ => input,
    }
}
