use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut map: HashSet<i32> = HashSet::new();

    for element in nums {
        if map.contains(&element) {
            return true;
        }

        map.insert(element);
    }

    false
}

#[cfg(test)]
mod test {
    use crate::contains_duplicate;

    #[test]
    fn test_duplicate() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
    }
}
