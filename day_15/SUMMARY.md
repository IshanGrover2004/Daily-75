# Day 15

## Question -> [AoC Day 06](https://adventofcode.com/2023/day/6)

## Intuition

Upon encountering the problem, the initial thoughts might be:

- This problem seems to involve races with time and distance parameters.
- The objective appears to be calculating certain conditions during races.
- There's a need to analyze button holds and time left to determine winning scenarios.

## Approach

### Part 1:

1. Parse the input to extract time and distance data for each race.
2. Create race objects using the parsed data.
3. Calculate winning scenarios based on certain conditions within each race.
4. Compute the maximum difference between the winning values.

### Part 2:

1. Parse the input to extract time and distance data for a single race.
2. Create a race object using the parsed data.
3. Calculate winning scenarios based on certain conditions within the race.
4. Compute the maximum difference between the winning values.

## Complexity

- **Time complexity:**
  - For Part 1: O(N\*M)
  - For Part 2: O(M)  
    --> Here, N is the number of lines in the input, and M is the maximum number of elements in a line.
- **Space complexity:**
  - For Part 1: O(N)
  - For Part 2: O(1) (constant space, as only a single race is processed at a time)  
    --> In Part 1, it stores the results of each line in a vector, while in Part 2, a HashMap is used to store elements.

## Code

[Click to view code](./AoC_day6/src/lib.rs)
