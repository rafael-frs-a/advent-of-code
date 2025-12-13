use regex::Regex;
use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

fn solve(line: &str) -> i32 {
    // E.g.: `[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}`
    // Pattern to locate `.##.`
    let indicator_pattern = Regex::new(r"\[([^\]]*)\]").unwrap();
    // Pattern to locate content within parenthesis
    let buttons_pattern = Regex::new(r"\(([^)]*)\)").unwrap();
    // Extract indices from `.##.` that are `#` into a HashSet. E.g.: {1, 2}
    let indicator_indices: HashSet<usize> = indicator_pattern.captures(line).unwrap()[1]
        .char_indices()
        .filter(|&(_, c)| c == '#')
        .map(|(idx, _)| idx)
        .collect();
    // Extract buttons into a vector of HashSet. E.g.: [{3}, {1, 3}, {2}, {2, 3}, {0, 2}, {0, 1}]
    let buttons: Vec<HashSet<usize>> = buttons_pattern
        .captures_iter(line)
        .map(|c| c[1].split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    let mut states = VecDeque::from([(indicator_indices, buttons, 0)]);

    while let Some((indicator_indices, buttons, press_count)) = states.pop_front() {
        for (idx, button) in buttons.iter().enumerate() {
            // Check if button contains any indicator indices
            if indicator_indices.is_disjoint(button) {
                continue;
            }

            let new_indicator_indices: HashSet<usize> = indicator_indices
                .symmetric_difference(button)
                .cloned()
                .collect();

            // Check if the target state has been achieved
            if new_indicator_indices.is_empty() {
                return press_count + 1;
            }

            let mut new_buttons = buttons.clone();
            new_buttons.remove(idx);
            states.push_back((new_indicator_indices, new_buttons, press_count + 1));
        }
    }

    // Should not happen
    i32::MAX
}

fn main() {
    let content = read_to_string("../input.txt").unwrap();
    let mut press_count = 0;

    for line in content.lines() {
        press_count += solve(line);
    }

    println!("{press_count}");
}
