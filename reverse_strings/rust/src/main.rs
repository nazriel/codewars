fn main() {
    println!("solution {:?}", solution("world"));
}

fn solution(phrase: &str) -> String {
    phrase.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_test() {
        assert_eq!(solution("world"), "dlrow");
    }
}
