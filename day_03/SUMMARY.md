# Day 03

## Question -> [Is array equivalent](https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/)

Two arrays of string is given, so check whether the arrays given forming the same string.
Eg. word1 = {"ab","c"}, word2 = {"a","bc"} => Resulting string is "abc" => Ans. `true`

## Intuition

My first thought was to join the elements of vector word and then compare the both resulting string. And this is exactly the answer.

## Approach

- Given two list of characters (word1 & word2).
- Traverse through the given 2 list of char and join the each element by method called `flatten` in rust resulting in a two strings.
- Compare the two strings if they are equal or not & return the boolean.

## Complexity

- Time complexity: $O(n)$

- Space complexity: $O(n)$

## Code

```rust
pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    let str1: String = word1.iter().map(|s| s.chars()).flatten().collect();
    let str2: String = word2.iter().map(|s| s.chars()).flatten().collect();

    str1==str2
}
```
