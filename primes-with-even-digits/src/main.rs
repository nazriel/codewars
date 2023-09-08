fn f(n: u32) -> u32 {

}

#[cfg(test)]
mod tests {
    use super::f;

    #[test]
    fn sample_tests() {
        assert_eq!(f(1000), 887);
        assert_eq!(f(1210), 1201);
        assert_eq!(f(10000), 8887);
        assert_eq!(f(500), 487);
        assert_eq!(f(487), 467);
    }
}
