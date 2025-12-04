# Day 2: Gift Shop

## Link

https://adventofcode.com/2025/day/2#part2

## Solution

To solve part two, we can adjust the brute force solution from part 1 to check for all factor lengths between 1 and `length / 2`, inclusive. If the number length is divisible by the factor length, we can mount the expected number that follows our rule of repeating segments. If the expected number matches the actual number, we add it to the result and stop evaluating other factor lengths to avoid double adding the number. We must also ensure the number length is greater than one, as the problem states that invalid numbers are composed of segments the repeat at least twice.

## Instructions

Add an `input.txt` file to the parent folder and run `cargo run` to see the result.
