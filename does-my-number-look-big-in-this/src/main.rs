fn narcissistic(input: u64) -> bool {
    let pow = input.to_string().len() as u32;

    dbg!(input);
    let mut sum = 0u64;
    for ch in input.to_string().chars() {
        let num = ch.to_digit(10).unwrap() as u64;
        let num_after_pow = num.pow(pow) as u64;

        if num_after_pow > input {
            return false;
        }

        sum += num_after_pow;

        if sum > input {
            return false;
        }
    }

    sum == input
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(input: u64, expected: bool) {
        let actual = narcissistic(input);
        assert_eq!(actual, expected, "\nIncorrect answer for n={}\nExpected: {expected}\nActual: {actual}", input)
    }

    #[test]
    fn basic_tests() {
        dotest(   7,  true);
        dotest( 371,  true);
        dotest( 122, false);
        dotest(4887, false);
        dotest(42678290603, true);
    }
}
