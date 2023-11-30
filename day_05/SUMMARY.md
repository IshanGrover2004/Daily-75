# Day XX

## Question -> [Valid Anagram](https://leetcode.com/problems/valid-anagram)

# Intuition

My First thought was to sort the string by characters & then compare if they are equal or not.

# Approach

- First check if given string length are same or not. If not => Ans is `false`.
- Iterate over both given strings characters and store the frequence/count of each character in a [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- Then compare both HashMap are equal or not. If not equal => Ans `false` else `true`.

# Complexity

- Time complexity: $O(n)$

- Space complexity: $O(n)$

# Code

## Method 1:

```rust
use std::collections::HashMap;

pub fn count_chars(str: &str) -> HashMap<char, i32> {
    let mut char_count = HashMap::new();
    str.chars().into_iter().for_each(|ch| {
        char_count
            .entry(ch)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

    char_count
}

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let s_char_count = count_chars(&s);
    let t_char_count = count_chars(&t);

    s_char_count == t_char_count
}
```

## Method 2:

```rust
pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
        return false;
    }

    let mut s_chars: Vec<char> = s.chars().collect();
    let mut t_chars: Vec<char> = t.chars().collect();

    s_chars.sort();
    t_chars.sort();

    s_chars == t_chars
}
```
