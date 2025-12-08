use std::collections::HashMap;
use std::fs::read_to_string;
use std::hash::Hash;

#[derive(PartialEq, Eq, Hash)]
struct Node {
    x: i64,
    y: i64,
    z: i64,
}

struct Edge {
    a_idx: usize,
    b_idx: usize,
    weight: f64,
}

impl Edge {
    fn new(a_idx: usize, b_idx: usize, nodes: &[Node]) -> Self {
        let a = &nodes[a_idx];
        let b = &nodes[b_idx];
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        let dz = a.z - b.z;

        // Euclidean distance
        let weight = ((dx * dx + dy * dy + dz * dz) as f64).sqrt();

        Self {
            a_idx,
            b_idx,
            weight,
        }
    }
}

struct DisjointSet {
    roots: HashMap<usize, usize>,
    n_islands: usize,
}

impl DisjointSet {
    fn new(n_nodes: usize) -> Self {
        let roots = (0..n_nodes).map(|n| (n, n)).collect();
        let n_islands = n_nodes;

        Self { roots, n_islands }
    }

    fn find(&self, node_idx: usize) -> usize {
        let mut node_idx = node_idx;

        loop {
            let root_idx = self.roots[&node_idx];

            if root_idx == node_idx {
                break node_idx;
            }

            node_idx = root_idx;
        }
    }

    fn union(&mut self, a_idx: usize, b_idx: usize) {
        let root_a_idx = self.find(a_idx);
        let root_b_idx = self.find(b_idx);

        if root_a_idx == root_b_idx {
            return;
        }

        self.roots.insert(root_a_idx, root_b_idx);
        self.n_islands -= 1;
    }
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
            edges.push(Edge::new(idx_a, idx_b, &nodes));
        }
    }

    edges.sort_by(|a, b| a.weight.total_cmp(&b.weight));

    // Merge edges
    let mut disjoint_set = DisjointSet::new(nodes.len());

    for edge in edges {
        disjoint_set.union(edge.a_idx, edge.b_idx);

        if disjoint_set.n_islands == 1 {
            println!("{}", nodes[edge.a_idx].x * nodes[edge.b_idx].x);
            break;
        }
    }
}
