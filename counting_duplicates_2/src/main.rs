use std::collections::HashMap;

fn count_duplicates(text: &str) -> u32 {
    let mut map = HashMap::new();

    for ch in text.to_ascii_lowercase().chars() {
        if !map.contains_key(&ch) {
            map.insert(ch, 1);
        } else {
            let cnt = map.get_mut(&ch).unwrap();
            *cnt += 1;
        }
    }

    map.into_values().filter(|x| *x > 1).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }

    #[test]
    fn test_abcdea() {
        assert_eq!(count_duplicates("abcdea"), 1);
    }

    #[test]
    fn test_indivisibility() {
        assert_eq!(count_duplicates("indivisibility"), 1);
    }
}
