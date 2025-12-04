use std::fs::read_to_string;

fn main() {
    let content = read_to_string("../input.txt").unwrap();
    let mut sum: u32 = 0;

    for line in content.lines() {
        let mut joltage = line.chars().take(2).collect::<String>();

        for new_digit in line.chars().skip(2) {
            let (first_digit, second_digit) = joltage.split_at(1);
            let current_joltage = joltage.parse::<u32>().unwrap();
            let first_option = format!("{first_digit}{new_digit}").parse::<u32>().unwrap();
            let second_option = format!("{second_digit}{new_digit}").parse::<u32>().unwrap();
            let max_joltage = current_joltage.max(first_option).max(second_option);
            joltage = max_joltage.to_string();
        }

        sum += joltage.parse::<u32>().unwrap();
    }

    println!("{sum}");
}
