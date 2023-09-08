fn valid_isbn10(isbn: &str) -> bool {
    if isbn.len() != 10 {
        return false;
    }

    if isbn[..9].chars().any(|c| !c.is_digit(10)) {
        return false;
    }

    let mut sum = 0u32;
    for (pos, ch) in isbn.chars().enumerate() {
        let number = match ch {
            '0'..='9' => ch.to_digit(10).unwrap_or(0),
            'X' => 10,
            _ => { return false; },
        };

        sum += (pos+1) as u32 * number;
    }

    sum % 11 == 0
}

#[cfg(test)]
mod tests {
    use super::valid_isbn10;

    fn dotest(isbn: &str, expected: bool) {
        let actual = valid_isbn10(isbn);
        assert!(actual == expected, "Test failed with isbn = {isbn}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn sample_tests() {
        dotest("1112223339", true);
        dotest("048665088X", true);
        dotest("1293000000", true);
        dotest("1234554321", true);
        dotest("1234512345", false);
        dotest("1293", false);
        dotest("X123456788", false);
        dotest("ABCDEFGHIJ", false);
        dotest("XXXXXXXXXX", false);
        dotest("123456789T", false);
    }
}
