mod tests;

use log::{debug, info};
use rand::Rng;

type Pos = (usize, usize); // Stored h,w

pub struct Board {
    cells: Vec<Vec<Cell>>, // Indexed h,w
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
            let w = rng.gen::<usize>() % self.width();
            let h = rng.gen::<usize>() % self.height();

            // Retry if bomb is too close to click or spot is already a bomb
            if w.abs_diff(c.1) <= 1 || h.abs_diff(c.0) <= 1 || self.get_cell((h, w)).bomb {
                continue;
            }

            debug!("Placing bomb at ({h}, {w})");
            self.cells[h][w].bomb = true;
            b -= 1;
        }

        // TODO: Calculate numbers
        for h in 0..self.height() {
            for w in 0..self.width() {
                self.cells[h][w].value = self
                    .nearby_cells((h, w))
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

    pub fn height(&self) -> usize {
        self.cells.len()
    }

    pub fn width(&self) -> usize {
        self.cells[0].len()
    }

    fn nearby_cells(&self, c: Pos) -> Vec<Pos> {
        // (c.1 - 1, c.0 - 1),
        // (c.1 - 1, c.0 - 0),
        // (c.1 - 1, c.0 + 1),
        // (c.1 - 0, c.0 - 1),
        // (c.1 - 0, c.0 + 1),
        // (c.1 + 1, c.0 - 1),
        // (c.1 + 1, c.0 - 0),
        // (c.1 + 1, c.0 + 1),
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
