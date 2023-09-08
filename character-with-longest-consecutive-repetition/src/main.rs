fn longest_repetition(s: &str) -> Option<(char, usize)> {
    if s.is_empty() {
        return None;
    }

    let mut longest = (s.chars().next().unwrap(), 1);
    let mut current = longest;

    for c in s.chars().skip(1) {
        if c == current.0 {
            current.1 += 1;
        } else {
            current = (c, 1);
        }

        if current.1 > longest.1 {
            longest = current;
        }
    }

    Some(longest)
}

#[cfg(test)]
mod tests {
    use super::longest_repetition;

    #[test]
    fn longest_at_the_beginning() {
        assert_eq!(longest_repetition(&"aaaabbb"), Some(('a', 4)));
    }

    #[test]
    fn longest_at_the_end() {
        assert_eq!(longest_repetition(&"abbbbb"), Some(('b', 5)));
        assert_eq!(longest_repetition(&"bbbaaabaaaa"), Some(('a', 4)));
    }

    #[test]
    fn longest_in_the_middle() {
        assert_eq!(longest_repetition(&"cbdeuuu900"), Some(('u', 3)));
    }

    #[test]
    fn multiple_longest() {
        assert_eq!(longest_repetition(&"aabb"), Some(('a', 2)));
        assert_eq!(longest_repetition(&"ba"), Some(('b', 1)));
    }

    #[test]
    fn single_character_only() {
        assert_eq!(longest_repetition(&"aaaaaa"), Some(('a', 6)));
    }

    #[test]
    fn empty_string() {
        assert_eq!(longest_repetition(&""), None);
    }
}
