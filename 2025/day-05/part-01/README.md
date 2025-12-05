# Day 5: Cafeteria

## Link

https://adventofcode.com/2025/day/5

## Solution

To solve this problem, we can simply check whether each ingredient ID is part of a given inclusive range and, if so, increase the result. That makes the solution `O(N*M)`, where `N` is the number of ranges and `M` is the number of ingredient IDs.

One way to optimize this solution would be to sort all ranges and, for each ingredient ID, search them with a binary search. This would make the solution `O(M log N)`. However, the previous solution is simpler and sufficient.

## Instructions

Add an `input.txt` file to the parent folder and run `cargo run` to see the result.
