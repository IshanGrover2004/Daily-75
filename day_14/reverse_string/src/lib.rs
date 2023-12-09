pub fn reverse_string(str_vec: &mut Vec<char>) {
    let (mut i, mut j) = (0, str_vec.len() as i32 - 1);

    while i < j {
        str_vec.swap(i as usize, j as usize);
        i += 1;
        j -= 1;
    }
}

pub fn easy_reverse(str_vec: &mut Vec<char>) {
    str_vec.reverse();
}

#[cfg(test)]
mod test {
    use crate::reverse_string;

    #[test]
    fn test_coverage() {
        let mut str_vec = vec!['h', 'e', 'l', 'l', 'o'];
        reverse_string(&mut str_vec);

        assert_eq!(str_vec, vec!['o', 'l', 'l', 'e', 'h']);
    }
}
