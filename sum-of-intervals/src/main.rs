fn sum_intervals(intervals: &[(i32, i32)]) -> i32 {
    let mut intervals = intervals.to_vec();
    intervals.sort_by(|a, b| a.0.cmp(&b.0));
    let mut sum = 0;
    let mut last = i32::MIN;
    for (start, end) in intervals {
        if start > last {
            sum += end - start;
            last = end;
        } else if end > last {
            sum += end - last;
            last = end;
        }
    }
    sum
}

#[cfg(test)]
mod sample_tests {
    use super::*;
    const ERR_MSG: &str = "\nYour result (left) did not match expected output (right).";

    #[test]
    fn non_overlapping_intervals() {
        assert_eq!(sum_intervals(&[(1, 5)]), 4, "{}", ERR_MSG);
        assert_eq!(sum_intervals(&[(1, 5), (6, 10)]), 8, "{}", ERR_MSG);
    }

    #[test]
    fn overlapping_intervals() {
        assert_eq!(sum_intervals(&[(1, 5), (1, 5)]), 4, "{}", ERR_MSG);
        assert_eq!(sum_intervals(&[(1, 4), (7, 10), (3, 5)]), 7, "{}", ERR_MSG);
    }

    #[test]
    fn large_intervals() {
        assert_eq!(
            sum_intervals(&[(-1_000_000_000, 1_000_000_000)]),
            2_000_000_000,
            "{}",
            ERR_MSG
        );
        assert_eq!(
            sum_intervals(&[(0, 20), (-100_000_000, 10), (30, 40)]),
            100_000_030,
            "{}",
            ERR_MSG
        );
    }

    #[test]
    fn misc() {
        assert_eq!(
            sum_intervals(&[(2, 3), (2, 6), (2, 4), (2, 9), (2, 5)]),
            7,
            "{}",
            ERR_MSG
        );
        assert_eq!(
            sum_intervals(&[
                (-100, -50),
                (76, 97),
                (24, 81),
                (-17, -14),
                (-2, 86),
                (-50, 46),
                (39, 87),
                (77, 96),
                (-77, -49),
                (-48, -16),
                (4, 88),
                (83, 88),
                (-58, -14),
                (-5, 67),
                (-70, -25),
                (17, 60),
                (16, 42),
                (72, 83)
            ]),
            197,
            "{}",
            ERR_MSG
        );
    }
}
