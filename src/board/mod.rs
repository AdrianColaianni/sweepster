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
        self.first_click = true;

        let mut rng = rand::thread_rng();

        // Place bombs, never next to first click
        let mut b = self.bombs;
        while b > 0 {
            let x = rng.gen::<usize>() % self.width();
            let y = rng.gen::<usize>() % self.height();

            // Retry if bomb is too close to click or spot is already a bomb
            if x.abs_diff(c.1) <= 1 || y.abs_diff(c.0) <= 1 || self.get_cell((y, x)).bomb {
                continue;
            }

            debug!("Placing bomb at ({x}, {y})");
            self.cells[y][x].bomb = true;
            b -= 1;
        }

        // TODO: Calculate numbers
        for y in 0..self.height() {
            for x in 0..self.width() {
                self.cells[x][y].value = self
                    .nearby_cells((y, x))
                    .iter()
                    .filter(|c| self.cells[c.1][c.0].bomb)
                    .count();
            }
        }
    }

    pub fn expose(&mut self, c: Pos) {
        if !self.first_click {
            self.fisrt_click(c);
        }

        self.cells[c.1][c.0].expose();

        if self.get_cell(c).value == 0 {
            self.reveal_around(c)
        }
    }

    pub fn plant_bomb(&mut self, c: Pos) {
        if !self.get_cell(c).is_covered() {
            return;
        }

        self.cells[c.1][c.0].state = CellState::Flagged;
    }

    pub fn get_cell(&self, c: Pos) -> &Cell {
        &self.cells[c.1][c.0]
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

    fn reveal_around(&mut self, c: Pos) {
        let mut n: Vec<Pos> = self
            .nearby_cells(c)
            .into_iter()
            .filter(|c| self.get_cell(*c).is_covered())
            .collect();

        while let Some(c) = n.pop() {
            self.expose(c);
            if self.get_cell(c).value == 0 {
                let mut n2 = self
                    .nearby_cells(c)
                    .into_iter()
                    .filter(|c| self.get_cell(*c).is_covered())
                    .collect();
                n.append(&mut n2);
            }
        }
    }
}

#[derive(Clone)]
pub struct Cell {
    pub state: CellState,
    bomb: bool,
    pub value: usize,
}

impl Cell {
    pub fn is_covered(&self) -> bool {
        self.state == CellState::Covered
    }

    pub fn expose(&mut self) {
        self.state = if self.bomb {
            CellState::Detonated
        } else {
            CellState::Empty
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            state: CellState::Covered,
            bomb: false,
            value: 0,
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
