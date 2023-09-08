fn next_smaller_number(n: u64) -> Option<u64> {
    let mut digits: Vec<u8> = n.to_string().chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
    let mut i = digits.len() - 1;
    while i > 0 && digits[i - 1] <= digits[i] {
        i -= 1;
    }
    if i <= 0 {
        return None;
    }
    let mut j = digits.len() - 1;
    while digits[j] >= digits[i - 1] {
        j -= 1;
    }
    digits.swap(i - 1, j);
    digits[i..].reverse();
    let r = digits.iter().fold(0, |acc, &x| acc * 10 + x as u64);
    if r < n {
        Some(r)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(Some(12), next_smaller_number(21));
        assert_eq!(Some(790), next_smaller_number(907));
        assert_eq!(Some(513), next_smaller_number(531));
        assert_eq!(None, next_smaller_number(1027));
        assert_eq!(Some(414), next_smaller_number(441));
    }
}
