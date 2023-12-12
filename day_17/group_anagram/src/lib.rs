use std::collections::HashMap;

fn sort_string(s: &str) -> String {
    let mut chars: Vec<_> = s.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}

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

#[cfg(test)]
mod test {
    use crate::group_anagrams;

    #[test]
    fn test_coverage() {
        assert_eq!(
            group_anagrams(vec!["eat", "tea", "tan", "ate", "nat", "bat"]),
            vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
        );
        assert_eq!(group_anagrams(vec![""]), vec![vec![""]]);
        assert_eq!(group_anagrams(vec!["a"]), vec![vec!["a"]]);
    }
}
