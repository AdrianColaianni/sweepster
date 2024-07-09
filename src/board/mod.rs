mod tests;

use log::{debug, info};
use rand::Rng;

type Pos = (usize, usize); // Stored or r,c

pub struct Board {
    cells: Vec<Vec<Cell>>, // Indexed or r,c
    bombs: usize,
    first_click: bool,
    pub auto_flag: bool,
    pub auto_reveal: bool,
}

impl Board {
    pub fn new(r: usize, c: usize, b: usize) -> Self {
        // Create board
        let cells = vec![vec![Cell::default(); c]; r];

        Self {
            cells,
            bombs: b,
            first_click: false,
            auto_flag: true,
            auto_reveal: true,
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
            if (col.abs_diff(c.1) <= 1 && row.abs_diff(c.0) <= 1) || self.get_cell((row, col)).bomb {
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
        let cell = cell.clone();

        if cell.value == 0 || self.is_cell_satsfied(c) {
            debug!("Revealing around {c:?}");
            self.reveal_around(c)
        }

        // Auto flag
        if self.auto_flag {
            let n: Vec<Pos> = self
                .nearby_cells(c)
                .into_iter()
                .filter(|c| {
                    let s = self.get_cell(*c).state;
                    s == CellState::Covered || s == CellState::Flagged || s == CellState::Detonated
                })
                .collect();
            debug!("Covered or flagged cells surrounding {c:?} is {}", n.len());
            if cell.value == n.len() {
                // All surrounding covered spaces must be bombs
                n.iter()
                    .for_each(|c| self.cells[c.0][c.1].state = CellState::Flagged);
                self.auto_reveal(n);
            }
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
    }

    pub fn get_cell(&self, c: Pos) -> &Cell {
        &self.cells[c.0][c.1]
    }

    fn get_cell_mut(&mut self, c: Pos) -> &mut Cell {
        &mut self.cells[c.0][c.1]
    }

    pub fn rows(&self) -> usize {
        self.cells.len()
    }

    pub fn columns(&self) -> usize {
        self.cells[0].len()
    }

    pub fn bombs_left(&self) -> isize {
        self.bombs as isize - self.cells
            .iter()
            .map(|r| r.iter().filter(|c| c.state == CellState::Flagged).count() as isize)
            .sum::<isize>()
    }

    fn nearby_cells(&self, c: Pos) -> Vec<Pos> {
        let (r, c) = c;
        assert!(r < self.rows() && c < self.columns());
        let mut n = vec![];

        let rgt0 = r > 0;
        let rltl = r < self.rows() - 1;

        if c > 0 {
            n.push((r, c - 1));
            if rgt0 {
                n.push((r - 1, c - 1));
            }
            if rltl {
                n.push((r + 1, c - 1));
            }
        }

        if rgt0 {
            n.push((r - 1, c));
        }
        if rltl {
            n.push((r + 1, c));
        }

        if c < self.columns() - 1 {
            n.push((r, c + 1));
            if rgt0 {
                n.push((r - 1, c + 1));
            }
            if rltl {
                n.push((r + 1, c + 1));
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
            self.nearby_cells(c).into_iter().for_each(|c| { self.is_cell_satsfied(c); });
            let cell = self.get_cell(c);
            if cell.value == 0 || self.is_cell_satsfied(c) {
                let mut n2 = self
                    .nearby_cells(c)
                    .into_iter()
                    .filter(|c| self.get_cell(*c).is_covered())
                    .collect();
                n.append(&mut n2);
            }
        }

    }

    fn auto_reveal(&mut self, n: Vec<Pos>) {
        if !self.auto_reveal {
            return;
        }

        n.into_iter()
            .for_each(|c| {
                self.nearby_cells(c).into_iter().for_each(|c| {
                    if self.is_cell_satsfied(c) {
                        self.reveal_around(c);
                    }
                });
            })
    }

    pub fn is_cell_satsfied(&mut self, c: Pos) -> bool {
        let cell = self.get_cell(c);
        if cell.satisfied {
            return true;
        }

        let f = self
            .nearby_cells(c)
            .into_iter()
            .filter(|c| self.get_cell(*c).state == CellState::Flagged)
            .count();

        if f == cell.value {
            self.get_cell_mut(c).satisfied = true;
            true
        } else {
            false
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
            satisfied: false
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
