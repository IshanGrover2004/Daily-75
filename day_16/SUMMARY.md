# Day 16

## Question -> [Replace elements](https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side)

## Intuition

First thought after seeing the question: "The code seems to be replacing each element in the array with the greatest element to its right."

## Approach

- The function takes a vector of integers arr.
- It iterates through arr in reverse order and tracks the maximum element encountered.
- For each element, it inserts the current maximum element (initialized as -1) at the beginning of the result vector.
- If the current element is greater than the current maximum, it updates the maximum element.
- Finally, it returns the resulting vector containing the replaced elements.

## Complexity

- Time complexity: $O(n)$
  -> Linear time because it iterates through the array once.

- Space complexity: $O(n)$
  -> Linear space as it creates a result array of the same size as the input array.

## Code

```rust
pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut max_element = -1;
    arr.iter().rev().for_each(|ele| {
        result.insert(0, max_element);
        if *ele > max_element {
            max_element = *ele;
        }
    });

    result
}
```
