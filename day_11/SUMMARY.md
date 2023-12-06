# Day 11

## Question -> [Valid Palindrome](https://leetcode.com/problems/valid-palindrome/)

## Intuition

My first thought was to collect all alphanumeric characters and collect in string & compare if it is equal to the reverse of string.

## Approach

#### Two pointers approach:

- Convert the input string to lowercase and extract only alphanumeric characters.
- Check if the string length is 0 or 1, return true immediately as it's considered a palindrome.
- Use two pointers (`i` and `j`) starting from both ends of the string.
- Iterate through the string with the pointers and compare characters from both ends.
- If characters at `i` and `j` positions don't match, return `false`; otherwise, continue comparing until the pointers meet or cross each other. Return `true` if the entire string is symmetric (palindrome).

## Complexity

- Time complexity: $O(n)$
  -> Since, the function iterates through the characters of the string once to normalize and filter the characters. The while loop that checks for palindrome properties by comparing characters from both ends iterates at most n/2 times where 'n' is the length of the filtered string.

- Space complexity: $O(1)$
  -> Since, only `i` & `j` variable are used for this approach => So space complexity will be constant.

## Code

```rust
pub fn is_palindrome(str: impl AsRef<str>) -> bool {
    let str = str
        .as_ref()
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<Vec<_>>();

    if str.len() == 0 || str.len() == 1 {
        return true;
    }

    let mut i = 0;
    let mut j = str.len() - 1;

    while i < j {
        if str[i] != str[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }

    true
}
```
