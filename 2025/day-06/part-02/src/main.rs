use std::fs::read_to_string;

fn solve_problem(problem: &[Vec<char>]) -> u64 {
    let operator = problem[problem.len() - 1][0];
    let mut result: u64 = if operator == '+' { 0 } else { 1 };

    // Let's iterate all columns in reverse order,
    // except the last one, which is expected to be a whitespace
    for col_idx in (0..problem[0].len() - 1).rev() {
        let mut operand = String::new();

        // Let's iterate all rows to make the operand,
        // except the last one, which is expected to be the operator
        for row in problem.iter().take(problem.len() - 1) {
            operand.push(row[col_idx]);
        }

        let operand: u64 = operand.trim().parse().unwrap();

        if operator == '+' {
            result += operand;
        } else {
            result *= operand;
        }
    }

    result
}

fn main() {
    let content = read_to_string("../input.txt").unwrap();
    let mut worksheet = vec![];

    for line in content.lines() {
        let mut characters: Vec<char> = line.chars().collect();
        // Let's add an extra whitespace char to simplify the logic below
        characters.push(' ');
        worksheet.push(characters);
    }

    let mut sum: u64 = 0;
    // Expected format after the problem's components are collected:
    // [
    //   ['1', '2', '3', ' '],
    //   [' ', '4', '5', ' '],
    //   [' ', ' ', '6', ' '],
    //   ['*', ' ', ' ', ' '],
    // ]
    let mut problem_components = vec![vec![]; worksheet.len()];

    for column_idx in 0..worksheet[0].len() {
        let mut is_all_whitespaces = true;

        for row_idx in 0..worksheet.len() {
            let character = worksheet[row_idx][column_idx];
            problem_components[row_idx].push(character);

            if character != ' ' {
                is_all_whitespaces = false;
            }
        }

        // Finished collecting problem. Let's solve it
        if is_all_whitespaces {
            sum += solve_problem(&problem_components);
            // Re-initialize problem's components
            problem_components = vec![vec![]; worksheet.len()];
        }
    }

    println!("{sum}");
}
