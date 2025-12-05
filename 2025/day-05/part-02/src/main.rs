use std::fs::read_to_string;

fn main() {
    let content = read_to_string("../input.txt").unwrap();
    let mut id_ranges = vec![];
    let mut count: u64 = 0;

    // Collect ranges
    for line in content.lines() {
        if line.trim().is_empty() {
            break;
        }

        let (lower, upper) = line.split_once('-').unwrap();
        let lower: u64 = lower.parse().unwrap();
        let upper: u64 = upper.parse().unwrap();
        id_ranges.push((lower, upper));
    }

    id_ranges.sort();

    // Merge overlapping ranges
    let mut merged_ranges = vec![];
    let (mut cur_lower, mut cur_upper) = id_ranges[0];

    for (lower, upper) in id_ranges.iter().skip(1) {
        if *lower <= cur_upper {
            cur_upper = cur_upper.max(*upper);
        } else {
            merged_ranges.push((cur_lower, cur_upper));
            cur_lower = *lower;
            cur_upper = *upper;
        }
    }

    merged_ranges.push((cur_lower, cur_upper));

    for (lower, upper) in merged_ranges {
        count += upper - lower + 1;
    }

    println!("{count}");
}
