# Day 4: Printing Department

## Link

https://adventofcode.com/2025/day/4

## Solution

To solve this problem, we simply walk through each space in the diagram and, if it's a roll of paper, count how many adjacent rolls of paper it has. If the count reaches the limit of four, we skip to the next space. We then add those that stay below the limit to the result.

## Instructions

Add an `input.txt` file to the parent folder and run `cargo run` to see the result.
