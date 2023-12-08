# Day 13

## Question -> [Search Insert position](https://leetcode.com/problems/search-insert-position)

## Intuition

My first thought was to use [binary search](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search) method built in rust which returns Result.  
If `target` is found Ok(index) and index is where target found, Else Err(index) and index is the nearest number could be.

## Approach

- Define a recursive binary search function to find the index of the target in the given sorted array of integers.
- Determine the start and end indices of the array.
- Use the binary search algorithm to recursively divide the array and narrow down the search space based on the target value.
- Return the index of the target if found, or the insertion index if not found.
- The insertion index is the position where the target would be placed to maintain the sorted order of the array.

## Complexity

- Time complexity: $O(log(n))$  
  -> The algorithm divides the array in half during each recursive call, leading to a logarithmic time complexity as it narrows down the search space.

- Space complexity: $O(log(n))$  
  -> The recursive calls occupy memory on the call stack proportional to the depth of the recursion, which follows a logarithmic pattern based on the number of elements in the array nn.

## Code

```rust
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    fn recursive_binary_search(nums: Vec<i32>, target: i32, start: i32, end: i32) -> i32 {
        if start > end {
            return start;
        }

        let mid = end - (end - start) / 2;
        match nums[mid as usize].cmp(&target) {
            std::cmp::Ordering::Equal => mid,
            std::cmp::Ordering::Less => recursive_binary_search(nums, target, mid + 1, end),
            std::cmp::Ordering::Greater => recursive_binary_search(nums, target, start, mid - 1),
        }
    }

    let (start, end) = (0, nums.len() as i32 - 1);
    recursive_binary_search(nums, target, start, end)
}
```
