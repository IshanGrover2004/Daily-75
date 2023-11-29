pub fn is_array_strings_are_equal(word1: Vec<&str>, word2: Vec<&str>) -> bool {
    let str1: String = word1.iter().map(|s| s.chars()).flatten().collect();
    let str2: String = word2.iter().map(|s| s.chars()).flatten().collect();

    str1 == str2
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::is_array_strings_are_equal;

    #[test]
    fn test_coverage() {
        assert_eq!(
            is_array_strings_are_equal(vec!["ab", "c"], vec!["a", "bc"]),
            true
        );
        assert_eq!(
            is_array_strings_are_equal(vec!["a", "cb"], vec!["ab", "c"]),
            false
        );
        assert_eq!(
            is_array_strings_are_equal(vec!["abc", "d", "defg"], vec!["abcddefg"]),
            true
        );
    }
}
