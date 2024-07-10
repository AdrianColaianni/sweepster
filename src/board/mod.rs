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
    pub fn new(r: usize, c: usize, bombs: usize, auto_flag: bool, auto_reveal: bool) -> Self {
        // Create board
        let cells = vec![vec![Cell::default(); c]; r];

        info!("Auto flag is {}", auto_flag);
        info!("Auto reveal is {}", auto_reveal);

        Self {
            cells,
            bombs,
            first_click: false,
            auto_flag,
            auto_reveal,
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
            if (col.abs_diff(c.1) <= 1 && row.abs_diff(c.0) <= 1) || self.get_cell((row, col)).bomb
            {
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

        if cell.value == 0 {
            self.reveal_empty(c);
        } else if self.is_cell_satsfied(c) {
            self.reveal_satisfied(c);
        } else if self.auto_flag {
            self.auto_flag(vec![c]);
        }
    }

    // Take a list of pos and try to plant flags
    fn auto_flag(&mut self, n: Vec<Pos>) {
        for c in n {
            let n: Vec<Pos> = self
                .nearby_cells(c)
                .into_iter()
                .filter(|c| !self.get_cell(*c).is_empty())
                .collect();

            let cell = self.get_cell_mut(c);
            debug!("Running auto_flag on {c:?}: {} surrounding", n.len());
            if n.len() == cell.value {
                debug!("Placing all bombs for {c:?}");
                n.into_iter().for_each(|c| {
                    self.place_bomb(c);
                });
            }
        }
    }

    fn place_bomb(&mut self, c: Pos) {
        self.get_cell_mut(c).state = CellState::Flagged;
    }

    pub fn toggle_bomb(&mut self, c: Pos) {
        let cell = &mut self.cells[c.0][c.1];

        // Toggle state
        if cell.state == CellState::Covered {
            cell.state = CellState::Flagged;
            if self.auto_reveal {
                self.auto_reveal(c);
            }
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
        self.bombs as isize
            - self
                .cells
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

    // Reveal EVERYTHING around a cell
    fn reveal_empty(&mut self, c: Pos) {
        debug!("Revealing empty cell {c:?}");

        assert_eq!(self.get_cell(c).value, 0);
        let mut e: Vec<Pos> = vec![c];
        let mut n: Vec<Pos> = vec![];
        let mut i = 0;

        // It's majorly fucked, majorly
        while i < e.len() {
            let c = e[i];
            for c in self.nearby_cells(c) {
                self.get_cell_mut(c).expose();
                if self.get_cell(c).value == 0 && !e.contains(&c) {
                    e.push(c);
                } else if !n.contains(&c) {
                    n.push(c);
                }
            }
            i += 1;
        }

        if self.auto_flag {
            self.auto_flag(n);
        }
    }

    // Reveal satisfied cell
    fn reveal_satisfied(&mut self, c: Pos) {
        debug!("Revealing satisfied cell {c:?}");

        let mut s = vec![c];
        let mut i = 0;

        while i < s.len() {
            let c = s[i];
            self.nearby_cells(c)
                .into_iter()
                .for_each(|c| {
                    let cell = self.get_cell_mut(c);
                    if cell.is_covered() {
                        cell.expose();
                        if self.is_cell_satsfied(c) && !s.contains(&c) {
                            s.push(c);
                        }
                    }
                });
            i += 1;
        }
    }

    fn auto_reveal(&mut self, n: Pos) {
        todo!()
    }

    pub fn is_cell_satsfied(&mut self, c: Pos) -> bool {
        let cell = self.get_cell(c);

        let f = self
            .nearby_cells(c)
            .into_iter()
            .filter(|c| self.get_cell(*c).state == CellState::Flagged)
            .count();

        if f == cell.value {
            if self.auto_reveal {
                self.expose(c)
            }
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
