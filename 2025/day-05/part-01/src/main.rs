use std::fs::read_to_string;

fn main() {
    let content = read_to_string("../input.txt").unwrap();
    let mut lines = content.lines();
    let mut id_ranges = vec![];

    // Collect ranges
    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }

        let (lower, upper) = line.split_once('-').unwrap();
        let lower: u64 = lower.parse().unwrap();
        let upper: u64 = upper.parse().unwrap();
        id_ranges.push((lower, upper));
    }

    let mut count = 0;

    for line in lines {
        let ingredient_id: u64 = line.parse().unwrap();

        for (lower, upper) in &id_ranges {
            if ingredient_id >= *lower && ingredient_id <= *upper {
                count += 1;
                break;
            }
        }
    }

    println!("{count}");
}
