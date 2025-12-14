use std::fs::read_to_string;

fn main() {
    let content = read_to_string("../input.txt").unwrap();
    let mut lines = content.lines();
    let mut shapes_areas = vec![];
    let mut shape_area = 0;

    // Count blocks
    for line in lines.by_ref().take(30) {
        if line.trim().is_empty() {
            shapes_areas.push(shape_area);
            shape_area = 0;
            continue;
        }

        shape_area += line.chars().filter(|&c| c == '#').count() as i32;
    }

    let mut positive_count = 0;

    for line in lines {
        let (grid_dimensions, shapes_amounts) = line.split_once(": ").unwrap();
        let (n_rows, n_columns) = grid_dimensions.split_once('x').unwrap();
        let n_rows: i32 = n_rows.parse().unwrap();
        let n_columns: i32 = n_columns.parse().unwrap();
        let shapes_amounts: Vec<i32> = shapes_amounts
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let total_area: i32 = shapes_areas
            .iter()
            .zip(shapes_amounts.iter())
            .map(|(&area, &amount)| area * amount)
            .sum();

        if total_area > n_rows * n_columns {
            continue;
        }

        let total_shapes_amount = shapes_amounts.iter().sum();

        if (n_rows / 3) * (n_columns / 3) >= total_shapes_amount {
            positive_count += 1;
            continue;
        }

        panic!("Input format not supported")
    }

    println!("{positive_count}");
}
