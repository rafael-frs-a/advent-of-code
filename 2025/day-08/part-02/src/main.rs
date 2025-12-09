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

struct DisjointSet<'a> {
    roots: HashMap<&'a Node, &'a Node>,
    n_islands: usize,
}

impl<'a> DisjointSet<'a> {
    fn new(nodes: &'a [Node]) -> Self {
        let roots = nodes.iter().map(|n| (n, n)).collect();
        let n_islands = nodes.len();

        Self { roots, n_islands }
    }

    fn find(&self, node: &'a Node) -> &'a Node {
        let mut node = node;

        loop {
            let root = self.roots[node];

            if root == node {
                break node;
            }

            node = root;
        }
    }

    fn union(&mut self, a: &'a Node, b: &'a Node) {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return;
        }

        self.roots.insert(root_a, root_b);
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
            let a = &nodes[idx_a];
            let b = &nodes[idx_b];
            edges.push(Edge::new(a, b));
        }
    }

    edges.sort_by(|a, b| a.weight.total_cmp(&b.weight));

    // Merge edges
    let mut disjoint_set = DisjointSet::new(&nodes);

    for edge in edges {
        disjoint_set.union(edge.a, edge.b);

        if disjoint_set.n_islands == 1 {
            println!("{}", edge.a.x * edge.b.x);
            break;
        }
    }
}
