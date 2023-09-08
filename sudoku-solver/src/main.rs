use std::collections::HashSet;

fn find_used_in_3_3_grid(grid: &[[u8; 9]; 9], row: usize, col: usize) -> HashSet<u8> {
    let mut used = HashSet::new();
    let row_start = row - row % 3;
    let col_start = col - col % 3;
    for i in row_start..row_start + 3 {
        for j in col_start..col_start + 3 {
            if grid[i][j] != 0 {
                used.insert(grid[i][j]);
            }
        }
    }
    used
}

fn find_used_in_row(grid: &[[u8; 9]; 9], row: usize) -> HashSet<u8> {
    let mut used = HashSet::new();
    for i in 0..9 {
        if grid[row][i] != 0 {
            used.insert(grid[row][i]);
        }
    }
    used
}

fn find_used_in_col(grid: &[[u8; 9]; 9], col: usize) -> HashSet<u8> {
    let mut used = HashSet::new();
    for i in 0..9 {
        if grid[i][col] != 0 {
            used.insert(grid[i][col]);
        }
    }
    used
}

fn find_available_numbers(set1: &HashSet<u8>, set2: &HashSet<u8>, set3: &HashSet<u8>) -> HashSet<u8> {
    let mut available = HashSet::new();
    for i in 1..10 {
        if !set1.contains(&i) && !set2.contains(&i) && !set3.contains(&i) {
            available.insert(i);
        }
    }
    available
}

fn solve(grid: &mut [[u8; 9]; 9]) -> bool {
    for row in (0..9).step_by(3) {
        for col in (0..9).step_by(3) {
            let used_in_grid = find_used_in_3_3_grid(grid, row, col);

            for dy in row..row + 3 {
                for dx in col..col + 3 {
                    if grid[dy][dx] != 0 {
                        continue;
                    }

                    let used_in_row = find_used_in_row(grid, dy);
                    let used_in_col = find_used_in_col(grid, dx);
                    let available = find_available_numbers(&used_in_grid, &used_in_row, &used_in_col);

                    for number in 1..10 {
                        if !available.contains(&number) {
                            continue;
                        }
                        grid[dy][dx] = number;

                        if solve(grid) {
                            return true;
                        }
                        grid[dy][dx] = 0;
                    }
                    return false;
                }
            }
        }
    }
    true
}

fn sudoku(puzzle: &mut [[u8; 9]; 9]) {
   solve(puzzle);
}


#[cfg(test)]
mod sample_tests {
    use super::sudoku;

    #[test]
    fn puzzle_1() {
        let mut puzzle = [
                [6, 0, 5, 7, 2, 0, 0, 3, 9],
                [4, 0, 0, 0, 0, 5, 1, 0, 0],
                [0, 2, 0, 1, 0, 0, 0, 0, 4],
                [0, 9, 0, 0, 3, 0, 7, 0, 6],
                [1, 0, 0, 8, 0, 9, 0, 0, 5],
                [2, 0, 4, 0, 5, 0, 0, 8, 0],
                [8, 0, 0, 0, 0, 3, 0, 2, 0],
                [0, 0, 2, 9, 0, 0, 0, 0, 1],
                [3, 5, 0, 0, 6, 7, 4, 0, 8],
            ];
        let solution = [
                [6, 1, 5, 7, 2, 4, 8, 3, 9],
                [4, 8, 7, 3, 9, 5, 1, 6, 2],
                [9, 2, 3, 1, 8, 6, 5, 7, 4],
                [5, 9, 8, 4, 3, 2, 7, 1, 6],
                [1, 3, 6, 8, 7, 9, 2, 4, 5],
                [2, 7, 4, 6, 5, 1, 9, 8, 3],
                [8, 4, 9, 5, 1, 3, 6, 2, 7],
                [7, 6, 2, 9, 4, 8, 3, 5, 1],
                [3, 5, 1, 2, 6, 7, 4, 9, 8],
            ];

        sudoku(&mut puzzle);
        assert_eq!(puzzle, solution, "\nYour solution (left) did not match the correct solution (right)");
    }

    #[test]
    fn puzzle_2() {
        let mut puzzle = [
                [0, 0, 8, 0, 3, 0, 5, 4, 0],
                [3, 0, 0, 4, 0, 7, 9, 0, 0],
                [4, 1, 0, 0, 0, 8, 0, 0, 2],
                [0, 4, 3, 5, 0, 2, 0, 6, 0],
                [5, 0, 0, 0, 0, 0, 0, 0, 8],
                [0, 6, 0, 3, 0, 9, 4, 1, 0],
                [1, 0, 0, 8, 0, 0, 0, 2, 7],
                [0, 0, 5, 6, 0, 3, 0, 0, 4],
                [0, 2, 9, 0, 7, 0, 8, 0, 0],
            ];
        let solution = [
                [9, 7, 8, 2, 3, 1, 5, 4, 6],
                [3, 5, 2, 4, 6, 7, 9, 8, 1],
                [4, 1, 6, 9, 5, 8, 3, 7, 2],
                [8, 4, 3, 5, 1, 2, 7, 6, 9],
                [5, 9, 1, 7, 4, 6, 2, 3, 8],
                [2, 6, 7, 3, 8, 9, 4, 1, 5],
                [1, 3, 4, 8, 9, 5, 6, 2, 7],
                [7, 8, 5, 6, 2, 3, 1, 9, 4],
                [6, 2, 9, 1, 7, 4, 8, 5, 3],
            ];

        sudoku(&mut puzzle);
        assert_eq!(puzzle, solution, "\nYour solution (left) did not match the correct solution (right)");
    }
}
