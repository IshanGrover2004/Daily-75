# Day 09

## Question -> [Is Subsequence](https://leetcode.com/problems/is-subsequence/)

## Intuition

My first thought was to use `contain` method and loop over the the characters and something related to it.

## Approach

- Convert both s and t into vectors of characters for easy traversal and comparison.
- Traverse through string t while maintaining an index, i, for string s.
- If the characters at indices i (from s) and j (from t) match, increment i. Continue this process through t.
- If the entire s string is iterated through successfully (i.e., i reaches its length), return true to confirm s is a subsequence of t.  
  If not, check if i matches the length of s to handle cases where s is a subsequence but occurs at the end of t.

## Complexity

- Time complexity: $O(n)$  
  -> This is because the function loops through each character in string t once.

- Space complexity: $O(n)$  
  -> Additional space is used to create character arrays for both s and t using chars().collect::<Vec<\_>>(). The space used is proportional to the lengths of the input strings.

## Code

```rust
pub fn is_subsequence(s: impl AsRef<str>, t: impl AsRef<str>) -> bool {
    let s = s.as_ref().chars().collect::<Vec<_>>();
    let t = t.as_ref().chars().collect::<Vec<_>>();

    let mut i = 0;

    for j in 0..t.len() {
        if i == s.len() {
            return true;
        } else if s[i] == t[j] {
            i += 1;
        }
    }

    s.len() == i
}
```
