use std::collections::HashSet;

struct Sudoku {
    data: Vec<Vec<u32>>,
}

impl Sudoku {
    /*
    Data structure dimension: NxN where N > 0 and √N == integer => done
    Rows may only contain integers: 1..N (N included) => done
    Columns may only contain integers: 1..N (N included) => done
    'Little squares' (3x3 in example above) may also only contain integers: 1..N (N included)
    */
    fn is_valid(&self) -> bool {
        if self.data.len() < 1 {
            return false;
        }

        for row in &self.data {
            if row.len() != self.data.len() {
                return false;
            }
        }

        if self.data.len() % 3 == 0 {
            for i in 0..3 {
                for j in 0..3 {
                    let mut set = HashSet::new();
                    for ii in i * 3..(i + 1) * 3 {
                        for jj in j * 3..(j + 1) * 3 {
                            let x = self.data[ii][jj];
                            if set.contains(&x) {
                                return false;
                            }
                            set.insert(x);
                        }
                    }
                }
            }
        }

        for row in &self.data {
            let mut max = 0;
            let mut set = HashSet::new();

            for column in row {
                if set.contains(column) {
                    return false;
                }
                if *column == 0 {
                    return false;
                }
                set.insert(column);
                if *column > max {
                    max = *column;
                }

                if max > row.len() as u32 {
                    return false;
                }
            }
        }

        for (x, _) in self.data[0].iter().enumerate() {
            let mut set = HashSet::new();
            for (y, _) in self.data.iter().enumerate() {
                let ch = self.data[y][x];
                if set.contains(&ch) {
                    return false;
                }
                set.insert(ch);
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_sudoku() {
        let good_sudoku_1 = Sudoku {
            data: vec![
                vec![7, 8, 4, 1, 5, 9, 3, 2, 6],
                vec![5, 3, 9, 6, 7, 2, 8, 4, 1],
                vec![6, 1, 2, 4, 3, 8, 7, 5, 9],
                vec![9, 2, 8, 7, 1, 5, 4, 6, 3],
                vec![3, 5, 7, 8, 4, 6, 1, 9, 2],
                vec![4, 6, 1, 9, 2, 3, 5, 8, 7],
                vec![8, 7, 6, 3, 9, 4, 2, 1, 5],
                vec![2, 4, 3, 5, 6, 1, 9, 7, 8],
                vec![1, 9, 5, 2, 8, 7, 6, 3, 4],
            ],
        };

        let good_sudoku_2 = Sudoku {
            data: vec![
                vec![1, 4, 2, 3],
                vec![3, 2, 4, 1],
                vec![4, 1, 3, 2],
                vec![2, 3, 1, 4],
            ],
        };

        let good_sudoku_3 = Sudoku {
            data: vec![vec![1]],
        };
        assert!(good_sudoku_1.is_valid());
        assert!(good_sudoku_2.is_valid());
        assert!(good_sudoku_3.is_valid());
    }

    #[test]
    fn bad_sudoku() {
        let bad_sudoku_1 = Sudoku {
            data: vec![
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            ],
        };

        let bad_sudoku_2 = Sudoku {
            data: vec![
                vec![1, 2, 3, 4, 5],
                vec![1, 2, 3, 4],
                vec![1, 2, 3, 4],
                vec![1],
            ],
        };

        let bad_sudoku_4 = Sudoku {
            data: vec![vec![2]],
        };

        let bad_sudoku_5 = Sudoku {
            data: vec![vec![0]],
        };
        assert!(!bad_sudoku_1.is_valid());
        assert!(!bad_sudoku_2.is_valid());
        assert!(!bad_sudoku_4.is_valid());
        assert!(!bad_sudoku_5.is_valid());
    }
}
