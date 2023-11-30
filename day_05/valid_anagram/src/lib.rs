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

#[cfg(test)]
mod test {
    use crate::is_anagram;

    #[test]
    fn test_coverage() {
        assert_eq!(
            is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
        assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false);
    }
}
