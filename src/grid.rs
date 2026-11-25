use std::{thread::sleep, time::Duration};

use crate::{
    HEIGHT, S_EMPTY, S_ENGINE, SPEED, WIDTH,
    types::{Cell, Dir, State},
};

pub struct Grid {
    pub cells: Vec<Cell>,
    next: Vec<Vec<Cell>>,
}

impl Grid {
    fn init_grid() -> Vec<Cell> {
        let mut grid = Vec::new();

        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if i == HEIGHT / 4 && (j == WIDTH - 1) {
                    grid.push(Cell {
                        state: State::ENGINE,
                        dir: Dir::Left,
                        speed: 1,
                    });
                } else if i == HEIGHT / 4 && (j == 0) {
                    grid.push(Cell {
                        state: State::ENGINE,
                        dir: Dir::Right,
                        speed: 3,
                    });
                } else {
                    grid.push(Cell {
                        state: State::EMPTY,
                        dir: Dir::Static,
                        speed: 0,
                    });
                }
            }
        }

        grid
    }

    pub fn new() -> Self {
        Self {
            cells: Grid::init_grid(),
            next: Vec::with_capacity(WIDTH * HEIGHT),
        }
    }

    pub fn print(&self) {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let el = self.cells.get(i * WIDTH + j).expect("Cell not found");
                if el.state == State::EMPTY {
                    print!("{} ", S_EMPTY);
                } else {
                    print!("{} ", S_ENGINE);
                }
            }
            println!("");
        }
    }

    pub fn next_gen(&mut self) {
        // self.next
        //     .clone_from(&self.cells.iter().map(|c| Vec::new().push(c)).collect::Vec<Cell>());

        for bucket in &mut self.next {
            bucket.clear();
        }
        unsafe {
            self.next.set_len(WIDTH * HEIGHT);
        }

        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let idx = i * WIDTH + j;
                let el = self.cells[idx];

                if el.state == State::ENGINE {
                    let (dy, dx) = el.dir.delta();
                    let ni = i as isize + dy * (el.speed as isize);
                    let nj = j as isize + dx * (el.speed as isize);

                    if ni >= 0 && ni < HEIGHT as isize && nj >= 0 && nj < WIDTH as isize {
                        let target_idx = (ni as usize) * WIDTH + (nj as usize);
                        self.next[target_idx].push(el);
                        // self.next[idx].state = State::EMPTY;
                        // self.next[idx].push(Cell::default());
                    }
                }
            }
        }

        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let idx = i * WIDTH + j;
                let el = self.next[idx].clone();
                if el.is_empty() {
                    self.cells[idx] = Cell::default();
                } else if el.len() == 1 {
                    self.cells[idx] = el[0];
                } else {
                    let cell = resolve_contenders(&el);
                    self.cells[idx] = cell;
                }
            }
        }

        // std::mem::swap(&mut self.cells, &mut self.next);
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

fn resolve_contenders(contenders: &[Cell]) -> Cell {
    let (mut vy, mut vx) = (0, 0);
    let mut engine_count = 0;

    for c in contenders {
        if c.state == State::ENGINE {
            engine_count += 1;
        }

        let (mut dy, mut dx) = c.dir.delta();
        (dy, dx) = (dy * c.speed as isize, dx * c.speed as isize);
        vy += dy;
        vx += dx;
    }

    if engine_count > 0 {
        if (vy == 0 && vx == 0) || (vy == vx) {
            return Cell::default();
        }

        let dir;
        let speed;
        let (abx, aby) = (vx.abs(), vy.abs());
        if aby > abx {
            if vy < 0 {
                dir = Dir::Up;
            } else {
                dir = Dir::Down;
            }
            speed = aby;
        } else {
            if vx < 0 {
                dir = Dir::Left;
            } else {
                dir = Dir::Right;
            }
            speed = abx;
        }

        let cell = Cell {
            state: State::ENGINE,
            dir,
            speed: speed as u8,
        };

        return cell;
    } else {
        return Cell::default();
    }
}
