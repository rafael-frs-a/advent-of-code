# Day 7: Laboratories

## Link

https://adventofcode.com/2025/day/7#part2

## Solution

To solve part two, we have to make some adjustments to the solution from part one. First, we keep track of how many possible beams there are at a given coordinate in the deque alongside the coordinate itself. By default, the initial coordinate starts with a beam count of one. If the new coordinate is a duplicate of the last in the deque, we sum their beam counts. Otherwise, we add the new coordinate and its corresponding beam count to the deque. Finally, we must move where we increment the final result from when we find a splitter to when the beam leaves the diagram at the bottom. Instead of increasing it by one, we increase it by the final beam count at that position.

## Instructions

Add an `input.txt` file to the parent folder and run `cargo run` to see the result.
