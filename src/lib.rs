use std::time::{SystemTime, UNIX_EPOCH};

/// The available maze generation tactics,
/// they were chosed based on their different strengts.
///
/// - Randomized `Dfs` => generates a maze biased towards long corridors.
///
/// - Randomized `Kruskal` => generates a maze biased towards short corridors.
///
/// - `Wilson`'s Algorithm => performs an unbiased maze generation.
pub enum GenTactic {
    Dfs,
    Kruskal,
    Wilson,
}

/// When you choose the width and height you should be aware that the width
/// and height height you provide will also account for all the walls, e.g.
/// if you use (3, 3) the initial state of the maze will look like the following:
///
/// ```markdown
/// +-------+
/// | 1 1 1 |
/// | 1 0 1 |
/// | 1 1 1 |
/// +-------+
/// ```
///
/// Where '0' represents an open cell (walkable) and '1' represents a wall.
pub struct Maze {
    width: usize,
    height: usize,
    grid: Vec<u8>, // TODO: change Vec<u8> for a type `Matrix`
}

/// This shouldn't be used for anything serious where the random
/// distribution & security matters.
/// TODO: make this generic
fn rand(end: u32) -> usize {
    let mut steps = 0;

    // Keep generating if you keep getting SystemTime errors
    loop {
        if steps == 100 {
            break 0;
        }

        let curr_time = SystemTime::now().duration_since(UNIX_EPOCH);
        if let Ok(rand) = curr_time {
            break (rand.subsec_nanos() % end) as usize;
        }

        steps += 1;
    }
}

impl Maze {
    /// The minimum width and height you should provide are both 3 and 3,
    /// if the values you provide are smaller than 3 `new` will return a 3x3 maze.
    pub fn new(width: usize, height: usize) -> Self {
        let width = std::cmp::max(3, width);
        let height = std::cmp::max(3, height);

        let mut grid = vec![1; width * height];
        for i in 1..height {
            for j in 1..width {
                if i % 2 == 1 && j % 2 == 1 {
                    grid[i * width + j] = 0;
                }
            }
        }

        Self {
            width,
            height,
            grid,
        }
    }

    /// Check if row `i`, column `j` exists in the grid.
    fn in_bounds(&self, i: usize, j: usize) -> bool {
        if i >= self.height || j >= self.width {
            false
        } else {
            true
        }
    }
    /// Sets the value of the cell at the `i`-th row, column `j` to `val`
    /// and returns if it found the cell to update. (e.g. self.at(...) didn't return [None])
    fn update(&mut self, i: usize, j: usize, val: u8) -> bool {
        if self.in_bounds(i, j) {
            self.grid[i * self.width + j] = val;
            true
        } else {
            false
        }
    }

    /// Returns the values of the cell at the row `i`, column `j` if it
    /// exists.
    pub fn at(&self, i: usize, j: usize) -> Option<u8> {
        if !self.in_bounds(i, j) {
            return None;
        }

        Some(self.grid[i * self.width + j])
    }

    pub fn gen(&mut self, tactic: GenTactic) {
        match tactic {
            GenTactic::Dfs => self.rand_dfs(),
            GenTactic::Kruskal => self.rand_kruskal(),
            GenTactic::Wilson => self.wilson(),
        }

        // TODO: self.find_exit();
        self.debug_rep();
    }

    /// This ia function ment to be used for debuggin the generated maze
    /// such that you can check if the generated maze is how it should look
    /// e.g. there are no weird patterns.
    pub fn debug_rep(&self) {
        // TODO: print into a log file
        for i in 0..self.width {
            for j in 0..self.height {
                if let Some(cell) = self.at(i, j) {
                    if cell == 1 {
                        eprint!("#");
                    } else {
                        eprint!(".");
                    }
                }
            }
            eprintln!();
        }
    }

    fn rand_dfs(&mut self) {
        // TODO: change this to Matrix
        let mut connected = vec![vec![false; self.width]; self.width];
        let dir_i: Vec<isize> = vec![1, -1, 0, 0];
        let dir_j: Vec<isize> = vec![0, 0, -1, 1];

        let mut stack: Vec<(usize, usize)> = Vec::from([(1, 1)]);
        while !stack.is_empty() {
            let curr_cell = stack.iter().last().unwrap();

            let mut dirs = Vec::new();
            while dirs.len() != 4 {
                // I don't want to include a crate just for this, I don't care
                // about the secuirty of maze generation :)
                let dir = rand(4);

                if !dirs.contains(&dir) {
                    dirs.push(dir);
                }
            }

            let mut broke_wall = false;
            for dir in dirs {
                let next_cell = (
                    curr_cell.0 as isize + 2 * dir_i[dir],
                    curr_cell.1 as isize + 2 * dir_j[dir],
                );

                if next_cell.0 < 0 || next_cell.1 < 0 {
                    continue;
                }

                let next_cell = (next_cell.0 as usize, next_cell.1 as usize);

                if self.in_bounds(next_cell.0, next_cell.1) {
                    if !connected[next_cell.0][next_cell.1] {
                        self.update(
                            (next_cell.0 as isize - dir_i[dir]) as usize,
                            (next_cell.1 as isize - dir_j[dir]) as usize,
                            0,
                        );
                        connected[next_cell.0][next_cell.1] = true;
                        stack.push(next_cell);
                        broke_wall = true;

                        break;
                    }
                }
            }

            if !broke_wall {
                stack.pop();
            }
        }
    }
    fn rand_kruskal(&mut self) {
        todo!();
    }

    fn wilson(&mut self) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dfs_gen_conne() {
        // TODO:
        // To test a RANDOM dfs maze gen we can check
        // if all cells at (i, j) where i and j are odd are connected
        // to at least another cell

        const WIDTH: usize = 11;
        const HEIGHT: usize = 11;

        let mut maze = Maze::new(WIDTH, HEIGHT);
        maze.gen(GenTactic::Dfs);
    }

    #[test]
    fn at_out_of_bounds() {
        const WIDTH: usize = 3;
        const HEIGHT: usize = 3;

        let maze = Maze::new(WIDTH, HEIGHT);
        assert_eq!(maze.at(2, 4), None);
        assert_eq!(maze.at(0, 4), None);
        assert_eq!(maze.at(5, 4), None);
        assert_eq!(maze.at(5, 2), None);
    }

    #[test]
    fn at_in_of_bounds() {
        const WIDTH: usize = 3;
        const HEIGHT: usize = 3;

        let maze = Maze::new(WIDTH, HEIGHT);
        assert_eq!(maze.at(0, 0), Some(1));
        assert_eq!(maze.at(1, 1), Some(0));
    }

    #[test]
    fn maze_size_validation() {
        let maze = Maze::new(3, 2);
        assert_eq!(maze.width, 3);
        assert_eq!(maze.height, 3);

        let maze = Maze::new(2, 3);
        assert_eq!(maze.width, 3);
        assert_eq!(maze.height, 3);

        let maze = Maze::new(1, 1);
        assert_eq!(maze.width, 3);
        assert_eq!(maze.height, 3);

        let maze = Maze::new(4, 3);
        assert_eq!(maze.width, 4);
        assert_eq!(maze.height, 3);
    }

    #[test]
    fn initial_grid() {
        const WIDTH: usize = 3;
        const HEIGHT: usize = 3;

        let maze = Maze::new(WIDTH, HEIGHT);

        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                if i % 2 == 1 && j % 2 == 1 {
                    assert_eq!(maze.at(i, j), Some(0));
                } else {
                    assert_eq!(maze.at(i, j), Some(1));
                }
            }
        }
    }
}
