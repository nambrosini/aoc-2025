pub fn rotate_90_clockwise<T: Clone>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    let rows = matrix.len();
    if rows == 0 {
        return vec![];
    }
    let cols = matrix[0].len();

    // Create a new matrix with swapped dimensions (cols becomes rows)
    let mut new_matrix = vec![vec![matrix[0][0].clone(); rows]; cols];

    for r in 0..rows {
        for c in 0..cols {
            // The logic: new_row = old_col, new_col = (old_height - 1) - old_row
            new_matrix[c][rows - 1 - r] = matrix[r][c].clone();
        }
    }

    new_matrix
}

pub fn rotate_90_anticlockwise<T: Clone>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    let rows = matrix.len();
    if rows == 0 {
        return vec![];
    }
    let cols = matrix[0].len();

    // Create a new matrix with swapped dimensions
    // The new height is 'cols', the new width is 'rows'
    let mut new_matrix = vec![vec![matrix[0][0].clone(); rows]; cols];

    for r in 0..rows {
        for c in 0..cols {
            // Logic:
            // The new row index is the inverse of the old column index.
            // The new column index is the old row index.
            new_matrix[cols - 1 - c][r] = matrix[r][c].clone();
        }
    }

    new_matrix
}

#[cfg(test)]
mod tests {
    use crate::util::matrices::{rotate_90_anticlockwise, rotate_90_clockwise};

    #[test]
    fn test_rotate_clockwise() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let actual = rotate_90_clockwise(&matrix);
        let expected = vec![vec![4, 1], vec![5, 2], vec![6, 3]];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_rotate_anticlockwise() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let actual = rotate_90_anticlockwise(&matrix);
        let expected = vec![vec![3, 6], vec![2, 5], vec![1, 4]];

        assert_eq!(actual, expected);
    }
}
