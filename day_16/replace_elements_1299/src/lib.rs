pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut max_element = -1;
    arr.iter().rev().for_each(|ele| {
        result.insert(0, max_element);
        if *ele > max_element {
            max_element = *ele;
        }
    });

    result
}

#[cfg(test)]
mod test {
    use crate::replace_elements;

    #[test]
    fn test_coverage() {
        assert_eq!(
            replace_elements(vec![17, 18, 5, 4, 6, 1]),
            vec![18, 6, 6, 6, 1, -1]
        );

        assert_eq!(replace_elements(vec![400]), vec![-1]);
    }
}
