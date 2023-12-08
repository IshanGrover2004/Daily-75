pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    fn recursive_binary_search(nums: Vec<i32>, target: i32, start: i32, end: i32) -> i32 {
        if start > end {
            return start;
        }

        let mid = end - (end - start) / 2;
        match nums[mid as usize].cmp(&target) {
            std::cmp::Ordering::Equal => mid,
            std::cmp::Ordering::Less => recursive_binary_search(nums, target, mid + 1, end),
            std::cmp::Ordering::Greater => recursive_binary_search(nums, target, start, mid - 1),
        }
    }

    let (start, end) = (0, nums.len() as i32 - 1);
    recursive_binary_search(nums, target, start, end)
}

#[cfg(test)]
mod test {
    use crate::search_insert;

    #[test]
    fn test_coverage() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}
