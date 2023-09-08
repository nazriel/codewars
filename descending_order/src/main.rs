fn descending_order(x: u64) -> u64 {
    let mut v = vec![];
    let mut n = x;

    while n > 0 {
        v.push(n % 10);
        n /= 10;
    }
    v.sort();
    v.reverse();

    let mut res: u64 = 0;
    let mut pos = 0;
    while let Some(d) = v.pop() {
        res = res + (d * u64::pow(10, pos));
        pos += 1;
    }

    res
}

fn main() {
    assert_eq!(descending_order(0), 0);
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(descending_order(0), 0);
        assert_eq!(descending_order(1), 1);
        assert_eq!(descending_order(15), 51);
        assert_eq!(descending_order(1021), 2110);
        assert_eq!(descending_order(123456789), 987654321);
        assert_eq!(descending_order(145263), 654321);
        assert_eq!(descending_order(1254859723), 9875543221);
    }
}
