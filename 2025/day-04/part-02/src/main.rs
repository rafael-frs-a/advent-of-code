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

    let mut rolls_coordinates = vec![];

    // Collect coordinates with rolls of paper
    for (row_idx, row) in diagram.iter().enumerate() {
        for (col_idx, space) in row.iter().enumerate() {
            if *space == '@' {
                rolls_coordinates.push((row_idx, col_idx));
            }
        }
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

    loop {
        let mut new_rolls_coordinates = vec![];

        for (row_idx, col_idx) in &rolls_coordinates {
            let mut count_adjacent_rolls = 0;

            for (add_row, add_col) in directions {
                let new_row_idx = (*row_idx as i32) + add_row;
                let new_col_idx = (*col_idx as i32) + add_col;

                // Check if new coordinate is inside the diagram
                if new_row_idx < 0
                    || new_row_idx >= diagram.len() as i32
                    || new_col_idx < 0
                    || new_col_idx >= diagram[*row_idx].len() as i32
                {
                    continue;
                }

                if diagram[new_row_idx as usize][new_col_idx as usize] == '@' {
                    count_adjacent_rolls += 1;
                }

                if count_adjacent_rolls == 4 {
                    break;
                }
            }

            // Removable roll found
            if count_adjacent_rolls < 4 {
                diagram[*row_idx][*col_idx] = '.';
                count += 1;
            // Roll of paper is not removable. Try on the next iteration
            } else {
                new_rolls_coordinates.push((*row_idx, *col_idx));
            }
        }

        // Check if any rolls were removed. If not, break the loop
        if new_rolls_coordinates.len() == rolls_coordinates.len() {
            break;
        }

        rolls_coordinates = new_rolls_coordinates;
    }

    println!("{count}");
}
