use std::fs;
use std::io;

fn main() {
    let mut file_name = String::new();
    println!("Enter your file name:");

    let _n = io::stdin()
        .read_line(&mut file_name)
        .expect("Nothing entered, sir.");
    file_name = file_name.trim().to_string();

    // let mut full_file_name: String = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    // full_file_name.push_str(r"\");
    // full_file_name.push_str(&file_name);

    let contents: String = fs::read_to_string(file_name).expect("Failed to read file");

    let answer = part1(&contents);
    println! {"Answer: {answer}"}
}

fn part1(input: &str) -> i64 {
    let mut sum: i64 = 0;
    for line in input.lines() {
        println!("{line}");
        let numbers_vec: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
        let mut intermediate: String = numbers_vec.first().expect("No first digit").to_string();
        intermediate.push(*numbers_vec.last().expect("No last digit"));
        let line_sum = intermediate
            .parse::<i64>()
            .expect("Not a digit, my good lad.");
        sum += line_sum;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example() {
        let contents = fs::read_to_string("ex_test.txt").expect("Failed to read file");
        let result = part1(&contents);
        assert_eq!(result, 142);
    }
}
