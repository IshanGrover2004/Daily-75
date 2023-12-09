# Day 14

## Question -> [Reverse String](https://leetcode.com/problems/reverse-string/)

## Intuition

My first thought was to just apply [reverse](https://doc.rust-lang.org/std/primitive.slice.html#method.reverse) method on vector of characters.

## Approach

### Iterative Approach

- Initialize two pointers, 'i' and 'j', at the beginning and end of the string respectively.
- While 'i' is less than 'j', continue the loop to reverse the string. This condition ensures that the reversal occurs only until the pointers meet or cross each other.
- Swap the characters at positions 'i' and 'j' using the swap method of the vector, effectively reversing the elements at these positions.
- Increment 'i' to move towards the end of the string, and decrement 'j' to move towards the start, continuing the process until the pointers converge.
- Once 'i' becomes greater than or equal to 'j', the entire string has been reversed, completing the function execution.

#### Code

```rust
pub fn reverse_string(str_vec: &mut Vec<char>) {
    let (mut i, mut j) = (0, str_vec.len() as i32 - 1);

    while i < j {
        str_vec.swap(i as usize, j as usize);
        i += 1;
        j -= 1;
    }
}
```

### One liner -> [reverse](https://doc.rust-lang.org/std/primitive.slice.html#method.reverse) method

#### Code

```rust
pub fn reverse_string(str_vec: &mut Vec<char>) {
    str_vec.reverse();
}
```

## Complexity

- Time complexity: $O(n)$  
  -> The time complexity is linear because the algorithm iterates through half of the input string length, swapping characters until reaching the middle.

- Space complexity: $O(1)$  
  -> The algorithm operates in constant space as it performs in-place reversal without using any additional data structures that scale with the input size; it only uses a constant amount of extra space for variables 'i', 'j', and temporary swaps.
