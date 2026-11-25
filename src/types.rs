#[derive(PartialEq, Eq, Clone, Copy)]
pub enum State {
    EMPTY,
    ENGINE,
    GENERATOR,
    GLUE,
    ROTATOR,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
    Static,
}

impl Dir {
    pub fn delta(self) -> (isize, isize) {
        match self {
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
            Dir::Static => (0, 0),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub state: State,
    pub dir: Dir,
    pub speed: u8,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            state: State::EMPTY,
            dir: Dir::Static,
            speed: 0,
        }
    }
}
