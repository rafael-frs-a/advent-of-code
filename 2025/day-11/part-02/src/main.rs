use petgraph::{Direction, algo::toposort, graphmap::DiGraphMap};
use std::collections::HashMap;
use std::fs::read_to_string;

fn make_count_from_source_dp<'a>(
    graph: &DiGraphMap<&'a str, ()>,
    topo: &[&'a str],
    source: &'a str,
) -> HashMap<&'a str, u64> {
    let mut dp: HashMap<&str, u64> = topo.iter().map(|&v| (v, 0)).collect();
    dp.insert(source, 1);

    for &node in topo {
        let count = dp[&node];

        if count == 0 {
            continue;
        }

        for neighbor in graph.neighbors_directed(node, Direction::Outgoing) {
            *dp.get_mut(&neighbor).unwrap() += count;
        }
    }

    dp
}

fn main() {
    let content = read_to_string("../input.txt").unwrap();
    let mut graph: DiGraphMap<&str, ()> = DiGraphMap::new();

    for line in content.lines() {
        let (device, connections) = line.split_once(": ").unwrap();
        let connections: Vec<&str> = connections.split_whitespace().collect();

        for connection in connections {
            graph.add_edge(device, connection, ());
        }
    }

    let topo = toposort(&graph, None).unwrap();
    let dp_svr = make_count_from_source_dp(&graph, &topo, "svr");
    let dp_dac = make_count_from_source_dp(&graph, &topo, "dac");
    let dp_fft = make_count_from_source_dp(&graph, &topo, "fft");

    let mut total_count: u64 = 0;
    total_count += dp_svr["dac"] * dp_dac["fft"] * dp_fft["out"];
    total_count += dp_svr["fft"] * dp_fft["dac"] * dp_dac["out"];
    println!("{total_count}");
}
