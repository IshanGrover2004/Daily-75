# Day 06

## Question -> [AoC 2023 Day 01](https://adventofcode.com/2023/day/1)

Given a whole input string, we wanted to calculate `calibration value` by parsing & manipulating the input lines.  
Input sample:
"abcone2threexyz" -> 13
"xtwone3four" -> 24
"4nineeightseven2" -> 42

Total Ans: 13+24+42 = `79`

## Approach

### Part 1

- Simple approach was to filter out those numbers which can be parsed from string digits("1") to integer(1).
- And then collect the filtered out integer in vector or array.
- Get the calibration value(Combination of first & last integer from vector) by following rules of it and collect in a vector.
- Find out the sum of calibration value from each input line.

### Part 2

- Iterate over the characters of input lines and check if current character is digit => if it is then directly parse and keep it safe somewhere for later.
- If it is not digit => Then take a string slice from currnet character to the end of the same line and check any of the number_word("one","two","five" etc) is in string slice. If it do then push the number found in vector.
- You will find after this process, the vector having parsed value is ready from where we can derive a calibration value(by combining start & end int value) for each line.
- Find out the sume of calibration value from each input line.

## Code

[Click to view code](./AoC_day1/src/lib.rs)

## Citations

Primeagen discord  
Divyansh bhaiya(senior from college)
