pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut ans: Vec<i32> = vec![0; n * 2];

    nums.into_iter().enumerate().for_each(|(idx, val)| {
        ans[idx] = val;
        ans[idx + n] = val;
    });

    ans
}

#[cfg(test)]
mod test {
    use crate::get_concatenation;

    #[test]
    fn test_coverage() {
        assert_eq!(get_concatenation(vec![1, 2, 1]), vec![1, 2, 1, 1, 2, 1]);
        assert_eq!(
            get_concatenation(vec![1, 3, 2, 1]),
            vec![1, 3, 2, 1, 1, 3, 2, 1]
        );
    }
}
