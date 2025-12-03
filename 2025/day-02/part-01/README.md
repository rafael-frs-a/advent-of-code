# Day 2: Gift Shop

## Link

https://adventofcode.com/2025/day/2

## Solution

We can solve this problem with brute force: listing all numbers in the inclusive ranges, checking if their length is even, and if their first half is equal to the second half. If so, we add the number to the result.

This problem can also be solved in linear time, where each range can be evaluated in `O(1)`. If, for example, the numbers we're evaluating in the range have a length of 6, we can infer each half is at least 100 and at most 999. Those numbers can be calculated with `10^(length / 2 - 1)` and `10^(length / 2) - 1`, respectively. We can also infer a base number that, when multiplied by each half, or factor, results in a 6-digit number. In this example, that number is 001001, or 1001 without the leading zeroes. It can be obtained by taking the max factor plus two. Then we can calculate the least and greatest factors that, when multiplied by the base number, result in a number within the range. That can be done with `ceil(lower_bound / base_number)` and `floor(upper_bound / base_number)`, respectively. We can take their sum with an arithmetic series, multiply it by the base number, and add it to the result. In the 6-digit numbers example, if we assume the least factor is 333 and the greatest factor is 336, that means the range contains the following invalid numbers: 333333, 334334, 335335, and 336336. We can express those with `333 * 1001 + 334 * 1001 + 335 * 1001 + 336 * 1001`, or `(333 + 334 + 335 + 336) * 1001`, where the first element corresponds to an arithmetic series, in which we only need the first and last factors to calculate it.

I decided to opt for the brute force approach as it was far simpler and it is still fast with Rust.

## Instructions

Add an `input.txt` file to the parent folder and run `cargo run` to see the result.
