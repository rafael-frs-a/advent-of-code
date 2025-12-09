# Day 8: Playground

## Link

https://adventofcode.com/2025/day/8#part2

## Solution

To solve part two, we could modify the solution from part one to add edges to the graph until all nodes have been added at least once. Then, we check whether the graph has just one island, adding another edge until it finally does. When that happens, we obtain the result by multiplying the x-coordinates of the two nodes in the last added edge. However, checking islands after each new edge is inefficient.

A better solution uses the Disjoint Set Union data structure. First, we initialize the disjoint set with all nodes. Each node has itself as its root, and the number of islands equals the number of nodes. For each edge, we merge its two nodes' islands by updating their respective roots. If their roots are the same, the nodes are already connected. If not, we assign the root of one node to the other's, connecting them and decreasing the number of islands by one. For example, consider the edges A-B, C-D, D-E, C-E, and B-C. Initially, A, B, C, D, and E are their own roots. On edge A-B, we update B's root (B) to have A's root (A) as its new root. On edge C-D, we update D's root (D) to have C's root (C) as its root. On D-E, we update D's root (C) to have E's root (E) as its root. On C-E, both C and E have the same root, E, so we do nothing. Finally, on B-C, we update C's root (E) to have B's root (A) as its root and achieve a single island. Note that despite A being E's root, they are not directly connected, as that is not part of the root idea.

## Instructions

Add an `input.txt` file to the parent folder and run `cargo run` to see the result.
