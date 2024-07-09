mod tests;

use log::{debug, info};
use rand::Rng;

type Pos = (usize, usize);

pub struct Board {
    cells: Vec<Vec<Cell>>, // Indexed h,w or y,x
    bombs: usize,
    first_click: bool,
}

impl Board {
    pub fn new(h: usize, w: usize, b: usize) -> Self {
        // Create board
        let cells = vec![vec![Cell::default(); w]; h];

        Self {
            cells,
            bombs: b,
            first_click: false,
        }
    }
    pub fn fisrt_click(&mut self, c: Pos) {
        info!("Running first_click()");

        let mut rng = rand::thread_rng();

        // Place bombs, never next to first click
        for _ in 0..self.bombs {
            let mut x = rng.gen::<usize>() % self.width();
            while x.abs_diff(c.1) <= 1 {
                x = rng.gen::<usize>() % self.width();
            }

            let mut y = rng.gen::<usize>() % self.height();
            while y.abs_diff(c.0) <= 1 {
                y = rng.gen::<usize>() % self.height();
            }

            debug!("Placing bomb at ({x}, {y})");

            self.cells[y][x].bomb = true;
        }

        // TODO: Calculate numbers
    }

    pub fn expose(&mut self, c: Pos) {
        if !self.first_click {
            self.fisrt_click(c);
        }

        self.cells[c.1][c.0].expose();
    }

    pub fn covered(&self, c: Pos) -> bool {
        self.cells[c.1][c.0].covered()
    }

    pub fn height(&self) -> usize {
        self.cells.len()
    }

    pub fn width(&self) -> usize {
        self.cells[0].len()
    }

    fn nearby_cells(&self, c: Pos) -> Vec<Pos> {
        // (c.0 - 1, c.1 - 1),
        // (c.0 - 1, c.1 - 0),
        // (c.0 - 1, c.1 + 1),
        // (c.0 - 0, c.1 - 1),
        // (c.0 - 0, c.1 + 1),
        // (c.0 + 1, c.1 - 1),
        // (c.0 + 1, c.1 - 0),
        // (c.0 + 1, c.1 + 1),
        let mut n = vec![];

        let c1gt0 = c.1 > 0;
        let c1lth = c.1 < self.height() - 1;

        if c.0 > 0 {
            n.push((c.0 - 1, c.1));
            if c1gt0 {
                n.push((c.0 - 1, c.1 - 1));
            }
            if c1lth {
                n.push((c.0 - 1, c.1 + 1));
            }
        }

        if c1gt0 {
            n.push((c.0, c.1 - 1));
        }
        if c1lth {
            n.push((c.0, c.1 + 1));
        }

        if c.0 < self.width() - 1 {
            n.push((c.0 + 1, c.1));
            if c1gt0 {
                n.push((c.0 + 1, c.1 - 1));
            }
            if c1lth {
                n.push((c.0 + 1, c.1 + 1));
            }
        }

        n
    }
}

#[derive(Clone)]
pub struct Cell {
    state: CellState,
    bomb: bool,
}

impl Cell {
    pub fn covered(&self) -> bool {
        self.state == CellState::Covered
    }

    pub fn expose(&mut self) {
        self.state = CellState::Empty
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            state: CellState::Covered,
            bomb: false,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Covered,
    Empty,
    Flagged,
    Detonated,
}
