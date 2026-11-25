use crate::grid::Grid;

mod grid;
mod types;

pub const SPEED: u64 = 500;
pub const WIDTH: usize = 25;
pub const HEIGHT: usize = 25;
pub const S_EMPTY: &str = "-";
pub const S_ENGINE: &str = "<";
pub const S_GEN: &str = "=";
pub const S_GLUE: &str = "#";
pub const S_ROT: &str = "@";

fn main() {
    let mut grid = Grid::new();
    grid.loop_gens();
}
