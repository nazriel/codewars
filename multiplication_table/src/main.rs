fn multiplication_table(len: usize) -> Vec<Vec<usize>> {
    let mut table = Vec::new();
    for row in 1..=len {
        let mut row_vec = Vec::new();
        for col in 1..=len {
            row_vec.push(row * col);
        }
        table.push(row_vec);
    }
    table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(multiplication_table(3), [[1,2,3], [2,4,6], [3,6,9]]);
    }
}
