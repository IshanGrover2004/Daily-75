# Day 04

## Question - [Array Concatenation](https://leetcode.com/problems/concatenation-of-array)

Given a array of integers, You have to create a new array which contain 2 times element of given array.  
Resulting array must have:

```rust
ans[i] == nums[i] and ans[i + n] == nums[i]
```

# Intuition

My first thought was to use any idiomatic rust method to solve this question in one liner. I find some one liner answer but it will not show what's going inside, so i did it manually.

# Approach

- Initialise a resulting empty vector of size = double the given vector & fill all element by 0.
- Start iterating the given vector and get `element` & `index` of each element.
- In each iteration, place the `element` at `index` and `index + length of nums`.
- The resulting vector is ready, so return it at last

# Complexity

- Time complexity: $O(n)$

- Space complexity: $O(n)$

# Code

```
pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut ans: Vec<i32> = vec![0; n * 2];

    nums.into_iter()
    .enumerate()
    .for_each(|(idx, val)| {
        ans[idx] = val;
        ans[idx + n] = val;
    });

    ans
}
```
