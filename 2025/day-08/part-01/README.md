# Day 8: Playground

## Link

https://adventofcode.com/2025/day/8

## Solution

In this problem, we can consider the junction boxes as nodes and the lines connecting a pair of junction boxes as the edges of a graph. To solve it, we first collect all nodes from the input. Then, we create the edges with all possible pairs of nodes, where the weight is the Euclidean distance between the two connected boxes. We then sort the list of edges by weight to get the 1,000 lines with the shortest distances and add those to a graph. Next, we need to find all islands in the graph and their sizes. To do that, we pop a node from the graph and use Depth-First Search (DFS) to walk all connected nodes, removing them from the graph and counting the island size in the process. As we finish walking the island, we add its size to a vector and start walking the next island. Once we walk all islands in the graph, we sort the vector of island sizes to get the three greatest sizes and multiply them to obtain the result.

## Instructions

Add an `input.txt` file to the parent folder and run `cargo run` to see the result.
