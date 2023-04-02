mod grid;
mod cell;
mod rule;

use grid::Grid;
use crate::cell::State;
use crate::rule::{Config, do_wolfram_rule};

fn main() {
    let epochs = 10 as usize;
    let mut grid = Grid::new(State::OFF, 100, 100);

    match grid.run_wolfram_rule(epochs, 1, None) {
        Ok(grid) => println!("{}", grid),
        Err(e) => println!("Failed: {}", e),
    }
}
