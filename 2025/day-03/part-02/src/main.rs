use std::fs::read_to_string;

fn main() {
    let content = read_to_string("../input.txt").unwrap();
    let mut sum: u64 = 0;

    for line in content.lines() {
        let mut joltage = line.chars().take(12).collect::<String>();

        for new_digit in line.chars().skip(12) {
            let current_joltage = joltage.parse::<u64>().unwrap();
            let mut candidates = vec![current_joltage];

            for idx in 0..joltage.len() {
                let (head, tail) = joltage.split_at(idx);
                let candidate = format!("{}{}{}", head, &tail[1..], new_digit);
                candidates.push(candidate.parse::<u64>().unwrap());
            }

            let max_joltage = candidates.iter().max().unwrap();
            joltage = max_joltage.to_string();
        }

        sum += joltage.parse::<u64>().unwrap();
    }

    println!("{sum}");
}
