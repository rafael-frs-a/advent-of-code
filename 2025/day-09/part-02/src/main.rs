use std::fs::read_to_string;

fn is_rectangle_inside_polygon(
    (x1, y1): (i32, i32),
    (x2, y2): (i32, i32),
    min_y: i32,
    y_horizontal_spans: &[(i32, i32)],
) -> bool {
    for y in y1.min(y2)..=y1.max(y2) {
        let (min_x, max_x) = y_horizontal_spans[(y - min_y) as usize];

        if x1.min(x2) < min_x || x1.max(x2) > max_x {
            return false;
        }
    }

    true
}

fn main() {
    let content = read_to_string("../input.txt").unwrap();
    let mut vertices = vec![];
    let (mut min_y, mut max_y) = (i32::MAX, 0);

    for line in content.lines() {
        let (x, y) = line.split_once(',').unwrap();
        let x: i32 = x.parse().unwrap();
        let y: i32 = y.parse().unwrap();
        min_y = min_y.min(y);
        max_y = max_y.max(y);
        vertices.push((x, y));
    }

    // For every `y`, let's store its min and max `x`
    let mut y_horizontal_spans = vec![(i32::MAX, 0); (max_y - min_y + 1) as usize];

    for idx in 0..vertices.len() {
        let (x1, y1) = vertices[idx];
        let (x2, y2) = vertices[(idx + 1) % vertices.len()];

        for y in y1.min(y2)..=y1.max(y2) {
            let idx = (y - min_y) as usize;
            let (cur_ax, cur_bx) = y_horizontal_spans[idx];
            y_horizontal_spans[idx] = (cur_ax.min(x1).min(x2), cur_bx.max(x1).max(x2));
        }
    }

    let mut max_area: i64 = 0;

    // Combine all possible different pairs of red tiles
    for idx_1 in 0..vertices.len() {
        for idx_2 in idx_1 + 1..vertices.len() {
            let (x1, y1) = vertices[idx_1];
            let (x2, y2) = vertices[idx_2];
            let area = ((x1 - x2).abs() + 1) as i64 * ((y1 - y2).abs() + 1) as i64;

            if area > max_area
                && is_rectangle_inside_polygon((x1, y1), (x2, y2), min_y, &y_horizontal_spans)
            {
                max_area = max_area.max(area);
            }
        }
    }

    println!("{max_area}");
}
