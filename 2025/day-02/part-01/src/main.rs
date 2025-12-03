use std::fs::read_to_string;

fn main() {
    let content = read_to_string("../input.txt").unwrap();
    let first_line = content.lines().next().unwrap();
    let mut sum: u64 = 0;

    for range in first_line.split(',') {
        let (lower_bound, upper_bound) = range.split_once('-').unwrap();
        let lower_bound = lower_bound.parse::<u64>().unwrap();
        let upper_bound = upper_bound.parse::<u64>().unwrap();

        for number in lower_bound..=upper_bound {
            let number_text = number.to_string();

            if number_text.len() % 2 == 1 {
                continue;
            }

            let (first_half, second_half) = number_text.split_at(number_text.len() / 2);

            if first_half == second_half {
                sum += number;
            }
        }
    }

    println!("{sum}");
}
