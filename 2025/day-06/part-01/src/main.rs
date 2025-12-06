use std::fs::read_to_string;

fn main() {
    let content = read_to_string("../input.txt").unwrap();
    let mut worksheet = vec![];

    for line in content.lines() {
        let operands: Vec<&str> = line.split_whitespace().collect();
        worksheet.push(operands);
    }

    let mut sum: u64 = 0;

    for problem_idx in 0..worksheet[0].len() {
        let operators = &worksheet[worksheet.len() - 1];
        let operator = operators[problem_idx];
        let mut problem_result: u64 = if operator == "+" { 0 } else { 1 };

        for operands in worksheet.iter().take(worksheet.len() - 1) {
            let operand: u64 = operands[problem_idx].parse().unwrap();

            if operator == "+" {
                problem_result += operand;
            } else {
                problem_result *= operand;
            }
        }

        sum += problem_result;
    }

    println!("{sum}");
}
