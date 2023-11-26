use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (idx, element) in nums.into_iter().enumerate() {
        let diff = target - element;

        match map.get(&diff) {
            Some(&second_idx) => {
                return vec![idx as i32, second_idx as i32];
            }
            None => {
                map.insert(element, idx);
            }
        };
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use crate::two_sum;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 0]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![2, 1]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![1, 0]);
    }
}
