use std::fs::read_to_string;

fn main() {
    let content = read_to_string("../input.txt").unwrap();
    let mut current_number = 50;
    let mut zero_count = 0;

    for line in content.lines() {
        let (dir, number) = line.split_at(1);
        let number = number.parse::<i32>().unwrap();

        let new_number = match dir {
            "R" => current_number + number,
            "L" => current_number - number,
            _ => panic!("Unknown direction"),
        };

        zero_count += (new_number / 100).abs();

        // Check if we crossed zero
        if new_number <= 0 && current_number > 0 {
            zero_count += 1;
        }

        // `current_number %= 100` does not work wih negative numbers as this problem expects
        current_number = new_number.rem_euclid(100);
    }

    println!("{zero_count}");
}
