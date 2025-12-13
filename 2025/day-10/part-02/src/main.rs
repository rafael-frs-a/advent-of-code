use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn make_parity_cases(
    buttons: &[Vec<i32>],
    n_lights: usize,
    parity_cases: &mut HashMap<Vec<i32>, Vec<(Vec<i32>, i32)>>,
) {
    // Make all possible combinations where a button is picked at most once
    // Generates `2^buttons.len()` combinations
    let combinations: Vec<Vec<Vec<i32>>> = (0..=buttons.len())
        .flat_map(|k| buttons.iter().cloned().combinations(k))
        .collect();

    for combination in &combinations {
        let mut base_case = vec![0; n_lights];

        for button in combination {
            base_case = base_case
                .iter()
                .zip(button.iter())
                .map(|(joltage, button_increase)| joltage + button_increase)
                .collect();
        }

        let parity_case: Vec<i32> = base_case.iter().map(|s| s % 2).collect();
        parity_cases
            .entry(parity_case)
            .or_default()
            .push((base_case, combination.len() as i32));
    }
}

fn count_button_presses(
    joltages: &[i32],
    cache: &mut HashMap<Vec<i32>, i32>,
    parity_cases: &HashMap<Vec<i32>, Vec<(Vec<i32>, i32)>>,
) -> i32 {
    if let Some(&count) = cache.get(joltages) {
        return count;
    }

    let joltages_parity: Vec<i32> = joltages.iter().map(|j| j % 2).collect();

    let Some(base_cases) = parity_cases.get(&joltages_parity) else {
        return i32::MAX;
    };

    let mut count = i32::MAX;

    for (base_case, base_case_count) in base_cases {
        let mut contains_negative = false;
        let new_joltages: Vec<i32> = joltages
            .iter()
            .zip(base_case.iter())
            .map(|(j, b)| {
                let difference = j - b;
                contains_negative |= difference < 0;
                difference / 2
            })
            .collect();

        if contains_negative {
            continue;
        }

        let new_joltages_count =
            count_button_presses(&new_joltages, cache, parity_cases).saturating_mul(2);
        count = count.min(base_case_count.saturating_add(new_joltages_count));
    }

    count
}

fn solve(line: &str) -> i32 {
    // E.g.: `[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}`
    // Pattern to locate content within curly braces
    let joltage_pattern = Regex::new(r"\{([^\]]*)\}").unwrap();
    // Pattern to locate content within parenthesis
    let buttons_pattern = Regex::new(r"\(([^)]*)\)").unwrap();
    // Extract joltages into a vector. E.g.: [3, 5, 4, 7]
    let joltages: Vec<i32> = joltage_pattern.captures(line).unwrap()[1]
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    // Extract buttons into a vector of HashSet. E.g.: [{3}, {1, 3}, {2}, {2, 3}, {0, 2}, {0, 1}]
    let buttons: Vec<HashSet<usize>> = buttons_pattern
        .captures_iter(line)
        .map(|c| c[1].split(',').map(|n| n.parse().unwrap()).collect())
        .collect();
    // Make buttons a binary representation of their corresponding joltage indices
    // E.g.: [
    //   [0, 0, 0, 1],
    //   [0, 1, 0, 1],
    //   [0, 0, 1, 0],
    //   [0, 0, 1, 1],
    //   [1, 0, 1, 0],
    //   [1, 1, 0, 0],
    // ]
    let buttons: Vec<Vec<i32>> = buttons
        .into_iter()
        .map(|button| {
            (0..joltages.len())
                .map(|idx| if button.contains(&idx) { 1 } else { 0 })
                .collect()
        })
        .collect();

    let mut cache = HashMap::new();
    // Base case
    cache.insert(vec![0; joltages.len()], 0);
    let mut parity_cases = HashMap::new();
    make_parity_cases(&buttons, joltages.len(), &mut parity_cases);
    count_button_presses(&joltages, &mut cache, &parity_cases)
}

fn main() {
    let content = read_to_string("../input.txt").unwrap();
    let mut press_count = 0;

    for line in content.lines() {
        press_count += solve(line);
    }

    println!("{press_count}");
}
