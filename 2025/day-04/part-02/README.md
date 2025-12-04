# Day 4: Printing Department

## Link

https://adventofcode.com/2025/day/4#part2

## Solution

To solve part 2, we first collect all coordinates that contain a roll of paper. We then start a loop to progressively remove them. On each iteration, we get every coordinate that still has a roll of paper and count how many adjacent rolls of paper it has. If it has fewer than four, we update the diagram, removing the roll of paper and increasing the result. Otherwise, we add the coordinate back for the next iteration. Once no rolls of paper have been removed, we break the loop.

## Instructions

Add an `input.txt` file to the parent folder and run `cargo run` to see the result.
