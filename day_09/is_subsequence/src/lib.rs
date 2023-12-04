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

#[cfg(test)]
mod test {
    use crate::is_subsequence;

    #[test]
    fn test_coverage() {
        assert_eq!(is_subsequence("abc", "ahbgdc"), true);
        assert_eq!(is_subsequence("axc", "ahbgdc"), false);
        assert_eq!(is_subsequence("", "ahbgdc"), true);
    }
}
