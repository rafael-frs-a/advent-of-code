# Day 9: Movie Theater

## Link

https://adventofcode.com/2025/day/9#part2

## Solution

To solve part two, we first need to assume the orthogonal polygon made from the provided list of vertices only has horizontal concavities. That means we don't have to worry about concave shapes such as the one below:

```
............
...#XXXX#...
...X#XX#X...
...##..##...
............
```

Unfortunately, I wasn't able to plot the shape from my input without crashing due to high memory usage, so I had to infer that assumption from shared solutions on the internet.

With that, we can deduce that each `y` coordinate inside the polygon (row) has a continuous span of coordinates from `(x1, y)` to `(x2, y)` (columns). We then collect those horizontal spans and iterate over each possible rectangle. For each rectangle, we iterate the `y` span (rows). For each `y`, we check if the rectangle's `x` span (column range) fits inside the `y`'s horizontal span. Only if all checks for all `y`s pass is the rectangle inside the polygon, and we can use its area in the search for the largest area.

One optimization that significantly speeds up the solution is using a vector for the horizontal spans and initializing it with its final size. That prevents the overhead of dynamic memory allocation with other structures such as a hash map.

## Instructions

Add an `input.txt` file to the parent folder and run `cargo run` to see the result.
