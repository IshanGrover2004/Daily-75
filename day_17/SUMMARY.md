# Day 17

## Question -> [Group Anagram](https://leetcode.com/problems/group-anagrams/)

## Intuition

The code seems to group anagrams by sorting their characters and using a HashMap to store and retrieve the groups efficiently.

## Approach

- Convert input vector elements to a vector of string references.
- Initialize a HashMap to store sorted strings as keys and anagrams as values.
- Iterate through each string, sort it, and use the sorted string as a key in the HashMap.
- Update the HashMap: add strings to the corresponding key if it exists, otherwise insert a new key-value pair.
- Collect the values of the HashMap (anagram groups) into a vector and return.

## Complexity

- Time complexity: $O(n * k * log k)$  
  -> n is the number of strings, k is the maximum length of a string, and sorting each string takes O(k \* log k) time.

- Space complexity: $O(n * k)$  
  -> where n is the number of strings and k is the maximum length of a string.

## Code

```rust
pub fn group_anagrams(strs: Vec<impl AsRef<str>>) -> Vec<Vec<String>> {
    let strs: Vec<&str> = strs.iter().map(|str| str.as_ref()).collect();

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    strs.into_iter().for_each(|str| {
        let sorted_str = sort_string(&str);

        map.entry(sorted_str)
            .and_modify(|vec| vec.push(str.to_string()))
            .or_insert_with(|| vec![str.to_string()]);
    });

    map.into_values().collect::<Vec<_>>()
}

fn sort_string(s: &str) -> String {
    let mut chars: Vec<_> = s.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}
```
