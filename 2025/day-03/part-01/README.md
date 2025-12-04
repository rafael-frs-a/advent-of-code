# Day 3: Lobby

## Link

https://adventofcode.com/2025/day/3

## Solution

We can solve this problem with `O(N)` time as follows: for each line, or bank, we start with a joltage candidate made from the first two digits in the line. We then iterate through each digit starting from the third one. With that digit, we make two new candidates by combining it with the digits from the current joltage, taking the greatest of the three: the current joltage and the two candidates. Once we reach the end of the line, we have found the greatest joltage and can add it to the result.

## Instructions

Add an `input.txt` file to the parent folder and run `cargo run` to see the result.
