use std::collections::VecDeque;
use std::fs::read_to_string;

fn generate_coordinates(
    diagram: &[Vec<char>],
    (row_idx, col_idx): (usize, usize),
    add_row: i32,
    add_col: i32,
) -> Option<(usize, usize)> {
    // Safely apply sum
    let new_row_idx = i32::try_from(row_idx).ok()?.checked_add(add_row)?;
    let new_col_idx = i32::try_from(col_idx).ok()?.checked_add(add_col)?;

    // Parse to usize
    let new_row_idx = usize::try_from(new_row_idx).ok()?;
    let new_col_idx = usize::try_from(new_col_idx).ok()?;

    // Check if the new coordinate is inside the diagram
    diagram.get(new_row_idx)?.get(new_col_idx)?;

    // Return the result
    Some((new_row_idx, new_col_idx))
}

fn add_coordinate_to_deque(
    beams_coordinates: &mut VecDeque<(usize, usize)>,
    (row_idx, col_idx): (usize, usize),
) {
    // Let's check if the split coordinate isn't a duplicate entry
    if beams_coordinates.back() != Some(&(row_idx, col_idx)) {
        beams_coordinates.push_back((row_idx, col_idx));
    }
}

fn main() {
    let content = read_to_string("../input.txt").unwrap();
    let mut diagram = vec![];
    let mut beams_coordinates = VecDeque::new();
    let mut count: u32 = 0;

    for (row_idx, line) in content.lines().enumerate() {
        let characters: Vec<char> = line.chars().collect();

        // Check for the starting position (S) if it hasn't been found
        if beams_coordinates.is_empty() {
            let col_idx_result = characters.iter().position(|&c| c == 'S');

            if let Some(col_idx) = col_idx_result {
                beams_coordinates.push_back((row_idx, col_idx));
            }
        }

        diagram.push(characters);
    }

    while let Some(coordinate) = beams_coordinates.pop_front() {
        let new_coordinate_result = generate_coordinates(&diagram, coordinate, 1, 0);
        let Some((row_idx, col_idx)) = new_coordinate_result else {
            continue;
        };

        // If it's not a splitter, let's add the new coordinate for the next iteration
        if diagram[row_idx][col_idx] != '^' {
            add_coordinate_to_deque(&mut beams_coordinates, (row_idx, col_idx));
            continue;
        }

        // Splitter detected. Let's increase the counter and generate the new coordinates
        count += 1;

        for add_col in [-1, 1] {
            let new_coordinate_result =
                generate_coordinates(&diagram, (row_idx, col_idx), 0, add_col);

            if let Some((split_row_idx, split_col_idx)) = new_coordinate_result {
                add_coordinate_to_deque(&mut beams_coordinates, (split_row_idx, split_col_idx));
            };
        }
    }

    println!("{count}");
}
