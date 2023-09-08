fn main() {
    println!("Hello, world!");
}

fn digital_root(n: i64) -> i64 {
    let mut sum = n;
    loop {
        let mut x = sum; // if sum > 0 { sum } else { n };
        sum = 0;
        while x > 0 {
            let digit = x % 10;
            x /= 10;
            sum += digit;
        }

        if sum < 10 {
            break
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(digital_root(9), 9);
        assert_eq!(digital_root(16), 7);
        assert_eq!(digital_root(942), 6);
        assert_eq!(digital_root(132189), 6);
        assert_eq!(digital_root(493193), 2);
    }
}
