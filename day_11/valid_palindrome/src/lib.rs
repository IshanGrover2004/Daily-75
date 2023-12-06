// Two pointers approach

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

#[cfg(test)]
mod test {
    use crate::is_palindrome;

    #[test]
    fn test_coverage() {
        assert_eq!(is_palindrome("A man, a plan, a canal: Panama"), true);
        assert_eq!(is_palindrome("race a car"), false);
        assert_eq!(is_palindrome(" "), true);
    }
}
