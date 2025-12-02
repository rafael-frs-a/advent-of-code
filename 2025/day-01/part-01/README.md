# Day 1: Secret Entrance

## Link

https://adventofcode.com/2025/day/1

## Solution

In this puzzle, we must count how many times the arrow on the circular dial points to zero after processing each input line. The arrow starts at 50. Read each line; if the first character is "R," add the number that follows, and if it is "L," subtract it. To simulate the dial wrapping around, take the result modulo 100.

## Instructions

Add an `input.txt` file to the parent folder and run `cargo run` to see the result.
