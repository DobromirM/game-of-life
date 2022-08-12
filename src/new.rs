use crate::{HEIGHT, WIDTH};

pub type State = [[bool; WIDTH]; HEIGHT];


#[derive(Debug, Copy, Clone)]
struct Coord {
    value: usize,
    max: usize,
}

impl Coord {
    fn new(value: usize, max: usize) -> Self {
        Coord {
            value,
            max,
        }
    }

    fn increment(&mut self) {
        self.value += 1;

        if self.value >= self.max {
            self.value = 0;
        }
    }

    fn decrement(&mut self) {
        self.value = self.value.checked_sub(1).unwrap_or(self.max - 1);
    }
}

#[derive(Debug, Copy, Clone)]
struct Cell {
    x: Coord,
    y: Coord,
}

impl Cell {
    fn new(x: usize, y: usize) -> Self {
        Cell {
            x: Coord::new(x, HEIGHT),
            y: Coord::new(y, WIDTH),
        }
    }

    fn into_neighbors_iter(self) -> NeighborsIter {
        NeighborsIter::new(self)
    }
}


struct NeighborsIter {
    state: usize,
    cell: Cell,
}

impl NeighborsIter {
    fn new(init: Cell) -> Self {
        Self {
            state: 0,
            cell: init,
        }
    }
}

impl Iterator for NeighborsIter {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            0 => {
                self.cell.x.decrement();
                self.cell.y.decrement()
            }
            1 => { self.cell.y.increment() }
            2 => { self.cell.y.increment() }
            3 => { self.cell.x.increment() }
            4 => { self.cell.x.increment() }
            5 => { self.cell.y.decrement() }
            6 => { self.cell.y.decrement() }
            7 => { self.cell.x.decrement() }
            _ => { return None; }
        }
        self.state += 1;
        Some(self.cell)
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    pub state: State,
}

impl Board {
    pub fn new() -> Self {
        Board {
            state: [[false; WIDTH]; HEIGHT]
        }
    }

    pub fn from(state: State) -> Self {
        Board {
            state
        }
    }

    pub fn next_step(self) -> Board {
        let mut next_state = [[false; WIDTH]; HEIGHT];

        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                match self.count_neighbors(Cell::new(row, col)) {
                    3 => next_state[row][col] = true,
                    2 if self.state[row][col] => next_state[row][col] = true,
                    _ => {}
                }
            }
        }
        Board {
            state: next_state
        }
    }

    fn count_neighbors(&self, cell: Cell) -> u8 {
        let mut neighbours: u8 = 0;

        for cell in cell.into_neighbors_iter() {
            if self.state[cell.x.value][cell.y.value] {
                neighbours += 1;
            }
        }
        neighbours
    }
}