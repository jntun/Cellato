use std::fmt::{Debug, Display, Formatter, Write};
use std::ops::Div;
use crate::{cell::{Cell, State}, cell};

pub enum GridError {
    Failure(String),
}

impl Display for GridError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            GridError::Failure(failure) => {
                f.write_str(failure)
            },
        }
    }
}

pub struct Grid {
    width: usize,
    height: usize,
    board: Vec<Vec<Cell>>,
    cell_count: usize,
}

impl Grid {
    fn get_next_cell_id(&mut self) -> usize {
        let id = self.cell_count;
        self.cell_count += 1;
        id
    }

    pub fn new(initial_state: State, width: usize, height: usize) -> Self {
        let mut grid = Self {width, height, cell_count: 0, board: Vec::new()};
        for i in 0..height {
            let mut row: Vec<Cell> = Vec::new();
            for x in 0..width {
                grid.cell_count = i + x;
                if i == 0 && x == width.div(2 as usize) {
                    row.push(Cell::new(grid.cell_count, State::ON))
                } else {
                    row.push(Cell::default_grid_cell(grid.cell_count))
                }
            }
            grid.board.push(row);
        }
        grid
    }

    pub fn run_wolfram_rule(mut self, epochs: usize, rule: u8, initial_cells: Option<Vec<(Cell, usize)>>) -> Result<Self, GridError> {
        for i in 0..epochs+1 {
        }
        Ok(self)
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        if let Err(e) = str.write_str("Grid:") {
            return f.write_str(format!("Grid failed to display: \n\t{}", e).as_str())
        }
        for row in self.board.iter() {
            str.write_str(format!("\n{:?}", row).as_str()).expect("Failed printing grid rows.");
        }
        f.write_str(str.as_str())
    }
}



impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let decimal_places = (0..).take_while(|i| 10u64.pow(*i) <= (self.width-1) as u64).count();
        let mut str = String::new();
        for (height, row) in self.board.iter().enumerate() {
            str.write_str(format!("\n{:0width$}=", height, width=decimal_places).as_str()).expect("TODO: panic message");
            for (width, cell) in row.iter().enumerate() {
                str.write_str(format!("{}", cell).as_str()).expect("TODO: panic message");
            }
            str.write_str(format!("={:0width$}", height, width=decimal_places).as_str()).expect("TODO: panic message");
        }
        f.write_str(str.as_str())
    }
}