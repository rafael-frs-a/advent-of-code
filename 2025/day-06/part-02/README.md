# Day 6: Trash Compactor

## Link

https://adventofcode.com/2025/day/6#part2

## Solution

To solve part two, we can no longer consider consecutive whitespaces as a single separator. One way to solve this is by collecting all characters in the input as an `N x M` matrix, where `N` (rows) is the number of components (`N - 1` operands and `1` operator) and `M` (columns) is the number of problems. Then we start iterating the matrix by columns. For each column, we iterate the `N` rows, concatenating each character to its corresponding element in the vector of components. Once all collected characters in the matrix's column are whitespaces, that means we have finished collecting all components of the problem and can solve it. All `N` elements in the components vector are expected to have the same number of columns. We can iterate the columns in reverse order, and the rows of all elements except the last, to make the operands. We can also determine the operator from the last element of the components vector. With the operands and operator collected, we solve the problem and add it to the final result.

## Instructions

Add an `input.txt` file to the parent folder and run `cargo run` to see the result.
