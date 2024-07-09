mod tests;

use log::{debug, info};
use rand::Rng;

type Pos = (usize, usize); // Stored or r,c

pub struct Board {
    cells: Vec<Vec<Cell>>, // Indexed or r,c
    bombs: usize,
    first_click: bool,
}

impl Board {
    pub fn new(r: usize, c: usize, b: usize) -> Self {
        // Create board
        let cells = vec![vec![Cell::default(); c]; r];

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
            let col = rng.gen::<usize>() % self.columns();
            let row = rng.gen::<usize>() % self.rows();

            // Retry if bomb is too close to click or spot is already a bomb
            if col.abs_diff(c.1) <= 1 || row.abs_diff(c.0) <= 1 || self.get_cell((row, col)).bomb {
                continue;
            }

            debug!("Placing bomb at ({row}, {col})");
            self.cells[row][col].bomb = true;
            b -= 1;
        }

        // TODO: Calculate numbers
        for r in 0..self.rows() {
            for c in 0..self.columns() {
                self.cells[r][c].value = self
                    .nearby_cells((r, c))
                    .iter()
                    .filter(|c| self.cells[c.0][c.1].bomb)
                    .count();
            }
        }
    }

    pub fn expose(&mut self, c: Pos) {
        if !self.first_click {
            self.fisrt_click(c);
        }

        let cell = &mut self.cells[c.0][c.1];

        if cell.state == CellState::Flagged {
            return;
        }

        cell.expose();

        if cell.value == 0 || cell.satisfied {
            debug!("Revealing around {c:?}");
            self.reveal_around(c)
        }
    }

    pub fn toggle_bomb(&mut self, c: Pos) {
        let cell = &mut self.cells[c.0][c.1];

        // Toggle state
        if cell.state == CellState::Covered {
            cell.state = CellState::Flagged;
        } else if cell.state == CellState::Flagged {
            cell.state = CellState::Covered;
        }

        // Update bombs around
        self.nearby_cells(c)
            .into_iter()
            .for_each(|c| self.update_satisfaction(c));
    }

    pub fn get_cell(&self, c: Pos) -> &Cell {
        &self.cells[c.0][c.1]
    }

    pub fn rows(&self) -> usize {
        self.cells.len()
    }

    pub fn columns(&self) -> usize {
        self.cells[0].len()
    }

    fn nearby_cells(&self, c: Pos) -> Vec<Pos> {
        let (r,c) = c;
        assert!(r < self.rows() && c < self.columns());
        let mut n = vec![];

        let rgt0 = r > 0;
        let c1lth = r < self.rows() - 1;

        if c > 0 {
            n.push((c - 1, r));
            if rgt0 {
                n.push((c - 1, r - 1));
            }
            if c1lth {
                n.push((c - 1, r + 1));
            }
        }

        if rgt0 {
            n.push((c, r - 1));
        }
        if c1lth {
            n.push((c, r + 1));
        }

        if c < self.columns() - 1 {
            n.push((c + 1, r));
            if rgt0 {
                n.push((c + 1, r - 1));
            }
            if c1lth {
                n.push((c + 1, r + 1));
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
            let cell = self.get_cell(c);
            if cell.value == 0 || cell.satisfied {
                let mut n2 = self
                    .nearby_cells(c)
                    .into_iter()
                    .filter(|c| self.get_cell(*c).is_covered())
                    .collect();
                n.append(&mut n2);
            }
        }
    }

    fn update_satisfaction(&mut self, c: Pos) {
        let f = self
            .nearby_cells(c)
            .into_iter()
            .filter(|c| self.get_cell(*c).state == CellState::Flagged)
            .count();

        let cell = &mut self.cells[c.0][c.1];
        if f == cell.value {
            debug!("Cell ({}, {}) is satisfied", c.0, c.1);
            cell.satisfied = true;
        }
    }
}

#[derive(Clone)]
pub struct Cell {
    pub state: CellState,
    bomb: bool,
    pub value: usize,
    satisfied: bool,
}

impl Cell {
    pub fn is_covered(&self) -> bool {
        self.state == CellState::Covered
    }

    pub fn is_empty(&self) -> bool {
        self.state == CellState::Empty
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
            satisfied: false,
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
