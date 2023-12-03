pub fn length_of_last_word(string: String) -> i32 {
    string.split_whitespace().last().unwrap().len() as i32
}

#[cfg(test)]
mod test {
    use crate::length_of_last_word;

    #[test]
    fn test_coverage() {
        assert_eq!(length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(
            length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
        assert_eq!(length_of_last_word("luffy is still joyboy".to_string()), 6);
    }
}
