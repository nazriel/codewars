fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut result = Vec::new();
    let mut count = 0;
    for i in lst {
        if result.contains(i) {
            count += 1;
            if count <= n {
                result.push(*i);
            }
        } else {
            result.push(*i);
            count = 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(delete_nth(&[20,37,20,21], 1), vec![20,37,21]);
        assert_eq!(delete_nth(&[1,1,3,3,7,2,2,2,2], 3), vec![1, 1, 3, 3, 7, 2, 2, 2]);
    }
}
