fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    data.iter().map(|(age, hc)| {
        match (age, hc) {
            (55.., 8..) => "Senior".to_string(),
            _ => "Open".to_string()
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(open_or_senior(vec![(45, 12), (55,21), (19, -2), (104, 20)]), vec!["Open", "Senior", "Open", "Senior"]);
        assert_eq!(open_or_senior(vec![(3, 12), (55,1), (91, -2), (54, 23)]), vec!["Open", "Open", "Open", "Open"]);
        assert_eq!(open_or_senior(vec![(55, 7)]), vec!["Open"]);
    }
}
