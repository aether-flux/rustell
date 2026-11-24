use std::{thread::sleep, time::Duration};

#[allow(unused)]

const SPEED: u64 = 500;
const WIDTH: usize = 25;
const HEIGHT: usize = 25;
const S_EMPTY: &str = "-";
const S_ENGINE: &str = "<";
const S_GEN: &str = "=";
const S_GLUE: &str = "#";
const S_ROT: &str = "@";

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum State {
    EMPTY,
    ENGINE,
    GENERATOR,
    GLUE,
    ROTATOR,
}

#[derive(Clone, Copy)]
pub struct Cell {
    state: State,
    xdir: u8,
    ydir: u8,
}

// #[derive(Clone, Copy)]
pub struct Grid {
    pub cells: Vec<Cell>,
    next: Vec<Cell>,
}

impl Grid {
    fn init_grid() -> Vec<Cell> {
        let mut grid = Vec::new();

        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if i == 0 && j == 0 {
                    grid.push(Cell {
                        state: State::ENGINE,
                        xdir: 1,
                        ydir: 1,
                    });
                } else {
                    grid.push(Cell {
                        state: State::EMPTY,
                        xdir: 0,
                        ydir: 0,
                    });
                }
            }
        }

        grid
    }

    pub fn new() -> Self {
        Self {
            cells: Grid::init_grid(),
            next: Vec::new(),
        }
    }

    pub fn print(&self) {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let el = self.cells.get(i * WIDTH + j).expect("Cell not found");
                if el.state == State::EMPTY {
                    print!("{} ", S_EMPTY);
                } else {
                    print!("{} ", S_GLUE);
                }
            }
            println!("");
        }
    }

    pub fn next_gen(&mut self) {
        self.next.clone_from(&self.cells);

        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let idx = i * WIDTH + j;
                let el = self.cells[idx];

                if el.state == State::ENGINE {
                    let ni = i as isize + (el.ydir as i8) as isize;
                    let nj = j as isize + (el.xdir as i8) as isize;

                    if ni >= 0 && ni < HEIGHT as isize && nj >= 0 && nj < WIDTH as isize {
                        let target_idx = (ni as usize) * WIDTH + (nj as usize);
                        self.next[target_idx] = el;
                        self.next[idx].state = State::EMPTY;
                    }
                }
            }
        }

        std::mem::swap(&mut self.cells, &mut self.next);
    }

    pub fn loop_gens(&mut self) {
        print!("\x1b[2J");
        loop {
            self.next_gen();
            self.print();
            sleep(Duration::from_millis(SPEED));
            print!("\x1b[2J");
        }
    }
}

fn main() {
    let mut grid = Grid::new();
    grid.loop_gens();
}
