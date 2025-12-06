# Day 6: Trash Compactor

## Link

https://adventofcode.com/2025/day/6

## Solution

To solve this problem, we can collect all operands from each line into a vector by splitting elements by whitespace. Rust's `split_whitespace` already treats consecutive occurrences of whitespace as a single separator, so we don't have to handle empty strings in the result. This process should produce `N` vectors with `M` elements, where the last vector contains the operators, `+` or `*`, and the previous ones contain the operands. Then, we can solve each problem individually by iterating over `M` and adding their solutions to the final result.

## Instructions

Add an `input.txt` file to the parent folder and run `cargo run` to see the result.
