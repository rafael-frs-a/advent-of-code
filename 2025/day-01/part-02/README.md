# Day 1: Secret Entrance

## Link

https://adventofcode.com/2025/day/1#part2

## Solution

To solve part two, we can count how many times the number passed through zero by taking the sum from each line before applying modulo, dividing by 100, and using the absolute integer result. One edge case we should account for is crossing zero for the first time when moving left. We can detect that by checking if the sum is less than or equal to zero and the previous number is positive.

## Instructions

Add an `input.txt` file to the parent folder and run `cargo run` to see the result.
