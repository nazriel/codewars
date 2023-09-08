mod solution {
    pub fn range_extraction(a: &[i32]) -> String {
        let mut last: i32 = i32::MIN;
        let mut range_start: i32 = i32::MIN;

        let mut output = String::new();

        for (pos, number) in a.into_iter().enumerate() {
            let curr = (*number).to_string();
            if range_start == i32::MIN {
                range_start = *number;
            }

            if last == i32::MIN {
                last = *number;
                output.push_str(&curr);
                continue;
            }

            if number - last != 1 { // not a range
                if last != range_start {
                    // push end of last range series
                    if last - range_start == 1 {
                        output.push_str(",");
                    }
                    output.push_str(&last.to_string());
                }

                range_start = *number;
                last = *number;
                output.push_str(",");
                output.push_str(&curr);
            } else { // range
                if number - range_start == 2 {
                    output.push_str("-");
                }

                // handle case when we are ending iteration
                if pos == a.len() - 1 {
                    if !output.ends_with("-") {
                        output.push_str(",");
                    }
                    output.push_str(&curr);
                }
                last = *number;
            }
        }

        output
    }
}

// dbg!(&number, &last, &range_start);
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            solution::range_extraction(&[
                -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
            ]),
            "-6,-3-1,3-5,7-11,14,15,17-20"
        );
        assert_eq!(
            solution::range_extraction(&[
                -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20, 35, 37, 40
            ]),
            "-6,-3-1,3-5,7-11,14,15,17-20,35,37,40"
        );
        assert_eq!(
            solution::range_extraction(&[
                -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20, 35, 37, 40, 41,43,44,47,49,50,52,53
            ]),
            "-6,-3-1,3-5,7-11,14,15,17-20,35,37,40,41,43,44,47,49,50,52,53"
        );
        assert_eq!(
            solution::range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20]),
            "-3--1,2,10,15,16,18-20"
        );
    }
}
