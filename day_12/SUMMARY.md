# Day 12

## Question -> [Binary Search](https://leetcode.com/problems/binary-search/)

## Intuition

First thought was to make a lazy move and use while loop(non-recursive way) to solve this without any difficulties.

## Approach

### Recursive Approach

- The function implements a binary search algorithm to find the target value within a sorted array.
- It initializes `start` as 0 and `end` as the last index of the nums array.
- Using recursion, it calculates the `mid` index to divide the search range and compares the element at the `mid` index with the target value.
- If the target matches the element at `mid`, it returns the index. Otherwise, it recursively calls itself by adjusting the search range based on whether the target is greater or smaller than the element at `mid`.
- The function returns -1 if the search range (`start` to `end`) does not overlap, indicating that the target is not present in the array.

## Complexity

- Time complexity: $O(log(n))$
  -> The algorithm's time complexity is logarithmic due to its binary search nature, reducing the search space by half in each step.

- Space complexity: $O(1)$
  -> The algorithm uses a constant amount of extra space for variables, as it doesn't rely on additional data structures dependent on the input size.

## Code

```rust
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (start, end) = (0_i32, nums.len() as i32 - 1);

    return Self::recursive_search(&nums, start, end, target);
}

fn recursive_search(nums: &Vec<i32>, start: i32, end: i32, target: i32) -> i32 {
    if start <= end {
        let mid = start + (end - start) / 2;

        if nums[mid as usize] == target {
            return mid as i32;
        } else if target > nums[mid as usize] {
            return Self::recursive_search(nums, mid + 1, end, target);
        } else {
            return Self::recursive_search(nums, start, mid - 1, target);
        }
    }

    -1
}
```
