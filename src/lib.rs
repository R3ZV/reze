mod dsu;
mod matrix;
mod rand;

use dsu::Dsu;
use matrix::Matrix;
use std::{
    fs::{self, File},
    io::Write,
};

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
    grid: Matrix<u8>,
}

impl Maze {
    /// The minimum width and height you should provide are both 3 and 3,
    /// if the values you provide are smaller than 3 `new` will return a 3x3 maze.
    ///
    /// You should also take into account that both the width and the height
    /// should be odd. If you provide an invalid size the function will increase
    /// your sizes by 1 to make them odd.

    pub fn new(width: usize, height: usize) -> Self {
        let mut width = std::cmp::max(3, width);
        let mut height = std::cmp::max(3, height);

        if width % 2 == 0 {
            width += 1;
        }

        if height % 2 == 0 {
            height += 1;
        }

        let mut grid = Matrix::new(height, width, 1);
        for i in 1..height {
            for j in 1..width {
                if i % 2 == 1 && j % 2 == 1 {
                    grid.update(i, j, 0);
                }
            }
        }

        Self {
            width,
            height,
            grid,
        }
    }

    /// Generates a new maze based on the generation
    /// tactic provided.
    ///
    /// `gen` will mutate the current grid.
    pub fn gen(&mut self, tactic: GenTactic) {
        match tactic {
            GenTactic::Dfs => self.rand_dfs(),
            GenTactic::Kruskal => self.rand_kruskal(),
            GenTactic::Wilson => self.wilson(),
        }

        // TODO: self.find_exit();
    }

    /// This ia function ment to be used for debuggin the generated maze
    /// such that you can check if the generated maze is how it should look
    /// e.g. there are no weird patterns.
    pub fn debug_rep(&self, id: usize) {
        for i in 0..self.height {
            for j in 0..self.width {
                if let Some(cell) = self.grid.at(i, j) {
                    if cell == 1 {
                        eprintln!("#");
                    } else {
                        eprintln!(".");
                    }
                }
            }
            eprintln!();
        }
        eprintln!();
    }

    fn rand_dfs(&mut self) {
        let mut connected = Matrix::new(self.height, self.width, false);
        let dir_i: Vec<isize> = vec![1, -1, 0, 0];
        let dir_j: Vec<isize> = vec![0, 0, -1, 1];

        let mut start_cell = (rand::rand(self.height), rand::rand(self.width));
        while start_cell.0 % 2 != 1 || start_cell.1 % 2 != 1 {
            if start_cell.0 % 2 != 1 {
                start_cell.0 = rand::rand(self.height);
            }

            if start_cell.1 % 2 != 1 {
                start_cell.1 = rand::rand(self.width);
            }
        }

        let mut stack: Vec<(usize, usize)> = Vec::from([start_cell]);
        while !stack.is_empty() {
            let curr_cell = stack.iter().last().unwrap();

            let mut dirs = Vec::new();
            while dirs.len() != 4 {
                // I don't want to include a crate just for this, I don't care
                // about the secuirty of maze generation :)
                let dir = rand::rand(4);

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

                let next_has_conn = connected.at(next_cell.0, next_cell.1);
                if next_has_conn.is_none() {
                    continue;
                }

                if !next_has_conn.unwrap() {
                    self.grid.update(
                        (next_cell.0 as isize - dir_i[dir]) as usize,
                        (next_cell.1 as isize - dir_j[dir]) as usize,
                        0,
                    );
                    connected.update(next_cell.0, next_cell.1, true);
                    stack.push(next_cell);

                    broke_wall = true;
                    break;
                }
            }

            if !broke_wall {
                stack.pop();
            }
        }
    }
    fn rand_kruskal(&mut self) {
        let mut edges: Vec<[(isize, isize); 2]> =
            Vec::with_capacity(2 * (self.width / 2 * self.height / 2));

        for i in (1..self.height).step_by(2) {
            for j in (1..self.width).step_by(2) {
                // N
                edges.push([(i as isize, j as isize), (-1, 0)]);

                // E
                edges.push([(i as isize, j as isize), (0, 1)]);
            }
        }

        rand::shuffle(&mut edges);

        while !edges.is_empty() {
            let [curr_cell, dir] = edges.pop().unwrap();
            let next_cell = (curr_cell.0 + 2 * dir.0, curr_cell.1 + 2 * dir.1);

            if next_cell.0 < 0 || next_cell.1 < 0 {
                continue;
            }

            if !self
                .grid
                .in_bounds(next_cell.0 as usize, next_cell.1 as usize)
            {
                continue;
            }

            let wall = (curr_cell.0 + dir.0, curr_cell.1 + dir.1);

            let next_cell: usize = next_cell.0 as usize * self.width + next_cell.1 as usize;
            let curr_cell: usize = curr_cell.0 as usize * self.width + curr_cell.1 as usize;

            let mut dsu = Dsu::new(self.width * self.height);

            if !dsu.same_set(curr_cell, next_cell) {
                dsu.merge(curr_cell, next_cell);

                self.grid.update(wall.0 as usize, wall.1 as usize, 0);
            }
        }
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
        const WIDTH: usize = 11;
        const HEIGHT: usize = 11;

        let mut maze = Maze::new(WIDTH, HEIGHT);
        maze.gen(GenTactic::Dfs);

        let mut conn = Matrix::new(HEIGHT, WIDTH, false);

        let dir_i = [-1, 1, 0, 0];
        let dir_j = [0, 0, -1, 1];
        for i in (1..HEIGHT).step_by(2) {
            for j in (1..WIDTH).step_by(2) {
                for dir in 0..4 {
                    let cell = (
                        (i as isize + dir_i[dir]) as usize,
                        (j as isize + dir_j[dir]) as usize,
                    );

                    if maze.grid.at(cell.0, cell.1).unwrap() == 0 {
                        conn.update(i, j, true);
                    }
                }
            }
        }

        for i in (1..HEIGHT).step_by(2) {
            for j in (1..WIDTH).step_by(2) {
                assert_eq!(conn.at(i, j), Some(true));
            }
        }
    }

    #[test]
    fn kruskal_gen_conne() {
        const WIDTH: usize = 11;
        const HEIGHT: usize = 11;

        let mut maze = Maze::new(WIDTH, HEIGHT);
        maze.gen(GenTactic::Kruskal);

        let mut conn = Matrix::new(HEIGHT, WIDTH, false);

        let dir_i = [-1, 1, 0, 0];
        let dir_j = [0, 0, -1, 1];
        for i in (1..HEIGHT).step_by(2) {
            for j in (1..WIDTH).step_by(2) {
                for dir in 0..4 {
                    let cell = (
                        (i as isize + dir_i[dir]) as usize,
                        (j as isize + dir_j[dir]) as usize,
                    );

                    if maze.grid.at(cell.0, cell.1).unwrap() == 0 {
                        conn.update(i, j, true);
                    }
                }
            }
        }

        for i in (1..HEIGHT).step_by(2) {
            for j in (1..WIDTH).step_by(2) {
                assert_eq!(conn.at(i, j), Some(true));
            }
        }
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
        assert_eq!(maze.width, 5);
        assert_eq!(maze.height, 3);

        let maze = Maze::new(3, 4);
        assert_eq!(maze.width, 3);
        assert_eq!(maze.height, 5);

        let maze = Maze::new(8, 8);
        assert_eq!(maze.width, 9);
        assert_eq!(maze.height, 9);
    }

    #[test]
    fn grid_init() {
        for width in 3..50 {
            for height in 3..50 {
                let maze = Maze::new(width, height);

                for i in 0..maze.height {
                    for j in 0..maze.width {
                        if i % 2 == 1 && j % 2 == 1 {
                            assert_eq!(maze.grid.at(i, j), Some(0));
                        } else {
                            assert_eq!(maze.grid.at(i, j), Some(1));
                        }
                    }
                }
            }
        }
    }
}
