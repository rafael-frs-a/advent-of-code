use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let content = read_to_string("../input.txt").unwrap();
    let mut graph = HashMap::new();

    for line in content.lines() {
        let (device, connections) = line.split_once(": ").unwrap();
        let connections: Vec<&str> = connections.split_whitespace().collect();
        graph.insert(device, connections);
    }

    let mut out_count = 0;
    let mut to_visit = vec!["you"];

    while let Some(device) = to_visit.pop() {
        if device == "out" {
            out_count += 1;
            continue;
        }

        for connected_device in &graph[device] {
            to_visit.push(connected_device);
        }
    }

    println!("{out_count}");
}
