use std::collections::HashMap;
use std::fs::read_to_string;
use std::hash::Hash;

#[derive(PartialEq, Eq, Hash)]
struct Node {
    x: i64,
    y: i64,
    z: i64,
}

struct Edge<'a> {
    a: &'a Node,
    b: &'a Node,
    weight: f64,
}

impl<'a> Edge<'a> {
    fn new(a: &'a Node, b: &'a Node) -> Self {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        let dz = a.z - b.z;

        // Euclidean distance
        let weight = ((dx * dx + dy * dy + dz * dz) as f64).sqrt();

        Self { a, b, weight }
    }
}

fn pop_any<K, V>(map: &mut HashMap<K, V>) -> Option<(K, V)>
where
    K: Eq + Hash + Clone,
{
    let key = map.keys().next()?.clone();
    map.remove_entry(&key)
}

fn main() {
    let content = read_to_string("../input.txt").unwrap();
    let mut nodes = vec![];

    // Collect nodes
    for line in content.lines() {
        let coordinate: Vec<i64> = line.split(',').map(|n| n.parse().unwrap()).collect();
        let [x, y, z] = coordinate.try_into().unwrap();
        nodes.push(Node { x, y, z });
    }

    // Make edges
    let mut edges = vec![];

    for idx_a in 0..nodes.len() {
        for idx_b in idx_a + 1..nodes.len() {
            let a = &nodes[idx_a];
            let b = &nodes[idx_b];
            edges.push(Edge::new(a, b));
        }
    }

    edges.sort_by(|a, b| a.weight.total_cmp(&b.weight));

    // Make graph
    let mut graph: HashMap<&Node, Vec<&Node>> = HashMap::new();

    for edge in edges.iter().take(1_000) {
        graph.entry(edge.a).or_default().push(edge.b);
        graph.entry(edge.b).or_default().push(edge.a);
    }

    // Count islands
    let mut island_sizes = vec![];

    while let Some((node, connections)) = pop_any(&mut graph) {
        let mut nodes_to_visit = vec![(node, connections)];
        // Count the node that was removed from the graph in `pop_any`
        let mut island_size = 1;

        while let Some((node, connections)) = nodes_to_visit.pop() {
            // Avoid double counting nodes
            if graph.remove_entry(node).is_some() {
                island_size += 1;
            }

            for node in connections {
                if let Some(second_connections) = graph.get(node) {
                    nodes_to_visit.push((node, second_connections.clone()));
                }
            }
        }

        island_sizes.push(island_size);
    }

    island_sizes.sort_by(|a, b| b.cmp(a));

    let mut result: i64 = 1;

    for island_size in island_sizes.iter().take(3) {
        result *= island_size;
    }

    println!("{result}");
}
