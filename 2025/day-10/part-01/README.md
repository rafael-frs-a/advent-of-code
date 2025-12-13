# Day 10: Factory

## Link

https://adventofcode.com/2025/day/10

## Solution

To solve this problem, we should first realize that pressing the same button more than once is not an option because:

1. Our goal is to find the fewest number of button presses.
2. Pressing the same button twice results in the same state as not pressing the button. Pressing it three times is the same as pressing it twice and then once more, and so on.

Instead of moving from the initial state, where all indicator lights are off, and trying to achieve the target state provided by the input, going the other way around is simpler to implement and should take the same number of button presses.

With that, we can use a First-In, First-Out (FIFO) approach with a deque to check all options. We can define the initial indicator state as a hash set of indices matching `#`. Similarly, we can define each button as a hash set of indices matching the input. For each indicator state, we iterate over the buttons and, for each button that overlaps with the indices in the indicator, we update the indicator state, remove the button from the list of available buttons, increase the press count, and add it to the end of our deque. The moment we find an indicator state made from an empty hash set, we reach the problem's initial state and have found the minimum number of presses.

## Instructions

Add an `input.txt` file to the parent folder and run `cargo run` to see the result.
