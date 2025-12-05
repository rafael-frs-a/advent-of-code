# Day 5: Cafeteria

## Link

https://adventofcode.com/2025/day/5#part2

## Solution

To solve part two, we can add the difference between the upper bound and the lower bound of each inclusive range plus one to the result. That is equivalent to how many numbers there are in them. However, there are overlapping ranges in the input, and simply doing that would count the same numbers multiple times. To solve that, we can merge the overlapping ranges into a single range. First, we sort the list of ranges. Then, we initialize the current lower and upper bounds with the first element of the list. For each next range, if its lower bound is less than or equal to the current upper bound, that means it is an overlapping range. In that case, we update the current upper bound with the greater of itself and the range's upper bound. If the ranges do not overlap, we add the current bounds to the list of merged ranges and reinitialize them with the current range. Once all ranges have been merged and collected, we can iterate over them and apply the initial idea of adding the differences. Iterating the ranges is `O(N)`, but sorting them is `O(N log N)`, so the latter defines the solution's time complexity.

## Instructions

Add an `input.txt` file to the parent folder and run `cargo run` to see the result.
