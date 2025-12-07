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
    beams_coordinates: &mut VecDeque<(usize, usize, u64)>,
    (row_idx, col_idx, beam_count): (usize, usize, u64),
) {
    // Let's check if the coordinate is a duplicate entry
    // If so, we increase the beam count. If not, we add the entry
    let Some((last_row_idx, last_col_idx, last_beam_count)) = beams_coordinates.pop_back() else {
        beams_coordinates.push_back((row_idx, col_idx, beam_count));
        return;
    };

    if (last_row_idx, last_col_idx) == (row_idx, col_idx) {
        beams_coordinates.push_back((row_idx, col_idx, beam_count + last_beam_count));
    } else {
        beams_coordinates.push_back((last_row_idx, last_col_idx, last_beam_count));
        beams_coordinates.push_back((row_idx, col_idx, beam_count));
    }
}

fn main() {
    let content = read_to_string("../input.txt").unwrap();
    let mut diagram = vec![];
    let mut beams_coordinates = VecDeque::new();
    let mut count: u64 = 0;

    for (row_idx, line) in content.lines().enumerate() {
        let characters: Vec<char> = line.chars().collect();

        // Check for the starting position (S) if it hasn't been found
        if beams_coordinates.is_empty() {
            let col_idx_result = characters.iter().position(|&c| c == 'S');

            if let Some(col_idx) = col_idx_result {
                beams_coordinates.push_back((row_idx, col_idx, 1_u64));
            }
        }

        diagram.push(characters);
    }

    while let Some((row_idx, col_idx, beam_count)) = beams_coordinates.pop_front() {
        let new_coordinate_result = generate_coordinates(&diagram, (row_idx, col_idx), 1, 0);
        // Beam leaves the diagram at its bottom
        let Some((row_idx, col_idx)) = new_coordinate_result else {
            count += beam_count;
            continue;
        };

        // If it's not a splitter, let's add the new coordinate for the next iteration
        if diagram[row_idx][col_idx] != '^' {
            add_coordinate_to_deque(&mut beams_coordinates, (row_idx, col_idx, beam_count));
            continue;
        }

        for add_col in [-1, 1] {
            let new_coordinate_result =
                generate_coordinates(&diagram, (row_idx, col_idx), 0, add_col);

            if let Some((split_row_idx, split_col_idx)) = new_coordinate_result {
                add_coordinate_to_deque(
                    &mut beams_coordinates,
                    (split_row_idx, split_col_idx, beam_count),
                );
            };
        }
    }

    println!("{count}");
}
