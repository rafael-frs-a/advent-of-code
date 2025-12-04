use std::fs::read_to_string;

fn main() {
    let content = read_to_string("../input.txt").unwrap();
    let mut diagram = vec![];
    let mut count = 0;

    // Make 2D matrix from input
    for line in content.lines() {
        let row: Vec<char> = line.chars().collect();
        diagram.push(row);
    }

    let directions = [
        (0, 1),   // Right
        (1, 0),   // Down
        (0, -1),  // Left
        (-1, 0),  // Up
        (-1, 1),  // Up-right
        (1, 1),   // Down-right
        (1, -1),  // Down-left
        (-1, -1), // Up-left
    ];

    for row_idx in 0..diagram.len() {
        'columns: for col_idx in 0..diagram[row_idx].len() {
            // Not a roll of paper
            if diagram[row_idx][col_idx] != '@' {
                continue;
            }

            let mut count_adjacent_rolls = 0;

            for (add_row, add_col) in directions {
                let new_row_idx = (row_idx as i32) + add_row;
                let new_col_idx = (col_idx as i32) + add_col;

                // Check if new coordinate is inside the diagram
                if new_row_idx < 0
                    || new_row_idx >= diagram.len() as i32
                    || new_col_idx < 0
                    || new_col_idx >= diagram[row_idx].len() as i32
                {
                    continue;
                }

                if diagram[new_row_idx as usize][new_col_idx as usize] == '@' {
                    count_adjacent_rolls += 1;
                }

                if count_adjacent_rolls == 4 {
                    continue 'columns;
                }
            }

            count += 1;
        }
    }

    println!("{count}");
}
