# Day 08

## Question -> [last word length](https://leetcode.com/problems/length-of-last-word)

## Intuition

My first thought was to just split the word by whitespaces and iterate till last element and measure the length of that word and return it as a answer.

## Approach

- Split the given sentence by using `split_whitespace` method.
- Take out the last element using `last` method.
- Now we have our last element, measure the lenth of word.
- Return the length as answer.

## Complexity

- Time complexity: $O(n)$  
  Since, we used `split_whitespace` method so it takes a loop of n times. Otherwise all have constant time compexity

- Space complexity: $O(1)$  
  No extra variables is used for getting the answer. So it will give O(1) overall.

## Code

```rust
pub fn length_of_last_word(sentence: String) -> i32 {
    sentence.split_whitespace().last().unwrap().len() as i32
}
```
