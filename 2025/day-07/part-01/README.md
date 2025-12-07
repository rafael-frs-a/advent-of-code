# Day 7: Laboratories

## Link

https://adventofcode.com/2025/day/7

## Solution

To solve this problem, we can simulate a beam being emitted from the starting position and count how many times it splits. To do that, we keep track of the beams' coordinates in a deque, following a "First-in, First-out" (FIFO) approach. To avoid double-counting splitters, we must avoid adding duplicate coordinates to the deque. Due to the FIFO approach, we can ensure all coordinates at a given time are sorted. We can take advantage of that to check whether the new coordinate is a duplicate by comparing it with the last coordinate of the deque.

## Instructions

Add an `input.txt` file to the parent folder and run `cargo run` to see the result.
