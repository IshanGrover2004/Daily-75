# Day 02

## Question - [Contains Dupicate](https://leetcode.com/problems/contains-duplicate/)

Vector of integer is given and we have to check whether given vector contains any duplicate or not.
Eg. Vector = {1,2,3,1}
Answer is "true" because it contains "1" two times.

## Intuition

My first thought was to sort given vector and then check if element at index `i` and `i+1` is same then the answer will be true.

## Approach

- Initialise a empty [HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html).
- Iterate the vector to access each element.
- Check if the element is in HashSet => Duplicate exist => Answer: **True**  
  else => Insert element in HashSet and go to next iteration
- At last if program is still running => Means no duplicate => Answer: **False**

## Complexity

- Time complexity: $O(n)$

- Space complexity: $O(n)$

## Code

```rust
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;

    let mut map: HashSet<i32> = HashSet::new();

    for element in nums {
        if map.contains(&element) {
            return true;
        }

        map.insert(element);
    }

    false
}
```
