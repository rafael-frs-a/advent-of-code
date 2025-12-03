use std::fs::read_to_string;

fn main() {
    let content = read_to_string("../input.txt").unwrap();
    let first_line = content.lines().next().unwrap();
    let mut sum: u64 = 0;

    for range in first_line.split(',') {
        let (lower_bound, upper_bound) = range.split_once('-').unwrap();
        let upper_bound_len = upper_bound.len();
        let lower_bound = lower_bound.parse::<u64>().unwrap();
        let upper_bound = upper_bound.parse::<u64>().unwrap();

        'numbers: for number in lower_bound..=upper_bound {
            let number_text = number.to_string();

            if number_text.len() < 2 {
                continue;
            }

            for factor_len in 1..=(upper_bound_len / 2) {
                if number_text.len() % factor_len != 0 {
                    continue;
                }

                let (segment, _) = number_text.split_at(factor_len);
                let expected_number_text = segment.repeat(number_text.len() / factor_len);

                if number_text == expected_number_text {
                    sum += number;
                    continue 'numbers;
                }
            }
        }
    }

    println!("{sum}");
}
