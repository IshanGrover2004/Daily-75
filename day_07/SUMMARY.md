# Day XX

## Question -> [Day-2 Cube Conundrum](https://adventofcode.com/2023/day/2)

## Intuition
My first thought was to parse the each line into some vector and then check the possible ways by applying rules and get the IDs of possible cases.

## Approach

### Part One:
   - Parse the input to extract there game IDs and colour subsets of cubes shown in each game.
   - For each game, verify if the case is valid or not, by checking the current colour ball with the max colour ball present.
   - Keep track of possible games.
   - Sum up IDs of all possible games.

### Part Two:
   - Similar to Part One, parse the input to extract game IDs and subsets of cubes shown in each game.
   - Calculate the minimum number of cubes of each color required to make each game possible.
   - Calculate the power of each set of cubes.
   - Sum up these powers to get the final result.

## Complexity
Warning: My code is not very time & space eficient because it contain so much spliting and looping.  

- Time complexity: $O(n^3)$  

- Space complexity: $O(n)$

## Code
[Click to view code](./AoC_day2/src/lib.rs)

## Citations

[Some random person github](https://github.com/cursorweb/JavaAOC/blob/rust/src/aoc2023/day2/mod.rs)
