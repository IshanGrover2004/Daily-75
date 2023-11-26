# Day 01

## Question - [Two Sum](https://leetcode.com/problems/two-sum)

Given a list of integers (Eg. [2,7,11,15]) & a target sum integer value (Eg. 9).
We have to take 2 integers and add together & check if equal to the target, if equal then answer will be position of that 2 elements.  
Every vector must have exactly 1 sum pair equal to target.

## Intuition

My first thought was to do double loop and add each element and get the target value easily. But it is a brute force approach and having Time Complexity of O(n^2).

## Approach

Optimized solution -:

- I have used HashMap here, where all elements are set into it in the form of element & respective index as `key:value pair`.
- Iterate once the vector or list given.
- Subtract the current element to the target sum and find out difference(2nd element).
- Now we want the position of difference element in list. So we will use `get` method of hashmap to get the value(index) at key(element)
- Now its a maybe situation,
  1. maybe the element or key exist => If exist then we can give answer consisting position of 1st element and 2nd element postion.
  2. mabe the element or key not exist => Then just wait until correct element found

## Complexity

- Time complexity: O(n)

- Space complexity: O(1)

## Code

```rust
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut map = HashMap::new();

    for (idx, element) in nums.into_iter().enumerate() {
        let diff = target - element;

        match map.get(&diff) {
            Some(&second_idx) => {
                return vec![idx as i32, second_idx as i32];
            }
            None => {
                map.insert(element, idx);
            }
        };
    }

    unreachable!()
}
```

## Citations

[Neetcode](neetcode.io/problems)
