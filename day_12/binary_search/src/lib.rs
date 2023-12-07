pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (start, end) = (0_i32, nums.len() as i32 - 1);

    return recursive_search(&nums, start, end, target);
}

fn recursive_search(nums: &Vec<i32>, start: i32, end: i32, target: i32) -> i32 {
    if start <= end {
        let mid = start + (end - start) / 2;

        if nums[mid as usize] == target {
            return mid as i32;
        } else if target > nums[mid as usize] {
            return recursive_search(nums, mid + 1, end, target);
        } else {
            return recursive_search(nums, start, mid - 1, target);
        }
    }

    -1
}

#[cfg(test)]
mod test {
    use crate::search;

    #[test]
    fn test_coverage() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
