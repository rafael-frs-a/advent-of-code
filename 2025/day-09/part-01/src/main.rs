use std::fs::read_to_string;

fn main() {
    let content = read_to_string("../input.txt").unwrap();
    let mut coordinates = vec![];

    for line in content.lines() {
        let (x, y) = line.split_once(',').unwrap();
        let x: i32 = x.parse().unwrap();
        let y: i32 = y.parse().unwrap();
        coordinates.push((x, y));
    }

    let mut max_area: i64 = 0;

    // Combine all possible different pairs of red tiles
    for idx_a in 0..coordinates.len() {
        for idx_b in idx_a + 1..coordinates.len() {
            let (ax, ay) = coordinates[idx_a];
            let (bx, by) = coordinates[idx_b];
            let area = ((ax - bx).abs() + 1) as i64 * ((ay - by).abs() + 1) as i64;
            max_area = max_area.max(area);
        }
    }

    println!("{max_area}");
}
