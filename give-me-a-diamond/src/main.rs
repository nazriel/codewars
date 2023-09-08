fn print(n: i32) -> Option<String> {
    if n < 1 || n % 2 == 0 {
        return None;
    }

    let mut result = "".to_string();
    for i in 1..=n {
        let spaces = " ".repeat((n / 2 + 1 - i).abs() as usize);
        let stars = "*".repeat(n as usize - (spaces.len() * 2));
        result.push_str(&format!("{spaces}{stars}\n"));
    }
    Some(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_test() {
        // assert_eq!(print(3), Some(" *\n***\n *\n".to_string()));
        assert_eq!(print(5), Some("  *\n ***\n*****\n ***\n  *\n".to_string()));
        assert_eq!(print(-3), None);
        assert_eq!(print(2), None);
        assert_eq!(print(0), None);
        assert_eq!(print(1), Some("*\n".to_string()));
    }
}
