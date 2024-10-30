/// An abstraction over a 1D array to simulate how a matrix work
/// without having the extra vector to worry about.

#[derive(Debug)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    arr: Vec<T>,
}

impl<T: Clone + Copy> Matrix<T> {
    pub fn new(rows: usize, cols: usize, init_val: T) -> Self {
        Self {
            rows,
            cols,
            arr: vec![init_val; rows * cols],
        }
    }

    /// Check if row `i`, column `j` exists in the grid.
    pub fn in_bounds(&self, i: usize, j: usize) -> bool {
        if i >= self.rows || j >= self.cols {
            false
        } else {
            true
        }
    }

    /// Sets the value of the cell at the `i`-th row, column `j` to `val`
    /// and returns if it found the cell to update. (e.g. self.at(...) didn't return [None])
    pub fn update(&mut self, i: usize, j: usize, val: T) -> bool {
        if self.in_bounds(i, j) {
            self.arr[i * self.cols + j] = val;
            true
        } else {
            false
        }
    }

    /// Returns the values of the cell at the row `i`, column `j` if it
    /// exists.
    pub fn at(&self, i: usize, j: usize) -> Option<T> {
        if !self.in_bounds(i, j) {
            return None;
        }

        Some(self.arr[i * self.cols + j])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_out_of_bounds() {
        const WIDTH: usize = 3;
        const HEIGHT: usize = 3;

        let maze = Matrix::new(WIDTH, HEIGHT, 0);
        assert_eq!(maze.at(2, 4), None);
        assert_eq!(maze.at(0, 4), None);
        assert_eq!(maze.at(5, 4), None);
        assert_eq!(maze.at(5, 2), None);
    }

    #[test]
    fn at_in_of_bounds() {
        const WIDTH: usize = 3;
        const HEIGHT: usize = 3;

        let maze = Matrix::new(WIDTH, HEIGHT, 0);
        assert_eq!(maze.at(0, 0), Some(0));
        assert_eq!(maze.at(1, 1), Some(0));
    }
}
