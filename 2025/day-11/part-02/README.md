# Day 11: Reactor

## Link

https://adventofcode.com/2025/day/11#part2

## Solution

To solve part two, given the input makes a directed acyclic graph, we can find the result by adding the following two values:

1. The number of paths between `"svr"` and `"dac"`, times the number of paths between `"dac"` and `"fft"`, times the number of paths between `"fft"` and `"out"`.
2. The number of paths between `"svr"` and `"fft"`, times the number of paths between `"fft"` and `"dac"`, times the number of paths between `"dac"` and `"out"`.

To find the number of paths between two nodes, we can add the possible counts to each intermediate node as we reach them, similar to part two from day 7. However, the graph is not structured as a grid, so we can't visit nodes simply using "first in, first out." To solve this, we need to find the graph's topological order, which gives us the order in which we should visit nodes. To obtain that, we can make use of the [petgraph](https://crates.io/crates/petgraph) crate. Then, we can apply our initial idea of adding the possible paths to each node.

## Instructions

Add an `input.txt` file to the parent folder and run `cargo run` to see the result.
