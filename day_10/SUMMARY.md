# Day 10

## Question -> [AoC 2023 - Day 04](https://adventofcode.com/2023/day/4)

## Intuition

The solution may require parsing, comparison of numbers to compute the desired scores or counts. The overall approach could involve iterating through the input, parsing numbers, comparing sets, and appropriately tallying the scores or combinations.

## Approach

### Part1:

- Parses input lines to extract winning and personal numbers.
- Evaluates the count of matches between the winning and personal numbers using the `eval_points` function.
- Sums up the `scores` obtained from each line to calculate the final result.

### Part2:

- Similar to Part1, extracts winning and personal numbers from input lines.
- Computes the count of matching numbers between the winning and personal numbers using the matching_numbers function.
- Utilizes a HashMap (copies) to keep track of counts related to matched numbers in subsequent lines and their duplicates.
- Take the total count of matches based on the tracked duplicates.

## Complexity

- Time complexity: $O(N*M)$  
  -> N is the number of lines in the input and M is the maximum number of elements in a line.

- Space complexity: $O(N)$  
  -> Because, it stores the results of each line in a vector & in part 2 , HashMap is used to store elements.

## Code

[Click to view code](./AoC_day04/src/lib.rs)
