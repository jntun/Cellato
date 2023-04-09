use std::fmt::{Debug, Display, Formatter, Write};
use std::ops::Div;
use crate::{
    cell::{Cell, State},
    rule,
};

pub type InitialCellConfig = Vec<(State, usize)>;

pub enum GridError {
    Failure(String),
    InitialCellPosIsGreaterThanWidth(String),
    CouldntWrapNeighborhood(String),
    CouldntGetAdjacentCell(String, String),
}

pub struct Grid {
    width: usize,
    epochs: usize,
    board: Vec<Vec<Cell>>,
    cell_count: usize,
}

impl Grid {
    fn do_wolfram_row(prev_row: &Vec<Cell>, update_row: &mut Vec<Cell>, rule: u8) -> Result<(), GridError> {
        for ((i, cell), update_cell) in prev_row.iter().enumerate().zip(update_row) {
            let neighbor_lhs: &Cell;
            let neighbor_rhs: &Cell;
            if i == 0 {
                let Some(lhs) = prev_row.get(prev_row.len()-1) else {
                    return Err(GridError::CouldntWrapNeighborhood(String::from("left")))
                };
                let Some(rhs) = prev_row.get(i+1) else {
                    return Err(GridError::CouldntGetAdjacentCell(String::from("right"), format!("{:?}", cell)));
                };
                neighbor_lhs = lhs;
                neighbor_rhs = rhs;
            } else if i == prev_row.len()-1 {
                let Some(rhs) = prev_row.get(0) else {
                    return Err(GridError::CouldntWrapNeighborhood(String::from("right")))
                };
                let Some(lhs) = prev_row.get(i-1) else {
                    return Err(GridError::CouldntGetAdjacentCell(String::from("left"), format!("{:?}", update_cell)))
                };
                neighbor_rhs = rhs;
                neighbor_lhs = lhs;
            } else {
                let Some(lhs) = prev_row.get(i-1) else {
                    return Err(GridError::CouldntGetAdjacentCell(String::from("left"), format!("{:?}", update_cell)))
                };
                let Some(rhs) = prev_row.get(i+1) else {
                    return Err(GridError::CouldntGetAdjacentCell(String::from("right"), format!("{:?}", update_cell)))
                };
                neighbor_lhs = lhs;
                neighbor_rhs = rhs;
            }
            update_cell.state = rule::wolfram(rule, (&neighbor_lhs.state, &cell.state, &neighbor_rhs.state));
        }
        Ok(())
    }

    fn do_board_tick(&mut self, rule: u8) -> Result<(), GridError> {
        for i in 0..self.board.len() {
            if i == 0 {
                continue
            }
            /*
            let Some(prev_row) = self.board.get_mut(i-1) else {
                return Err(GridError::Failure(format!("Failed to get prev_row at {}", i-1)))
            };
            let Some(update_row) = self.board.get_mut(i) else {
                return Err(GridError::Failure(format!("Failed to get curr_row at {}", i)));
            };

            if let Err(e) = self.do_wolfram_row(prev_row, update_row, rule) {
                return Err(e);
            };
             */
            let (prev, update) = self.board.split_at_mut(i);
            let Some(prev_row) = prev.last() else {
                return Err(GridError::Failure(format!("Failed to get prev_row at {}", i)))
            };
            let Some(update_row) = update.first_mut() else {
                return Err(GridError::Failure(format!("Failed to get update_row at {}", i)));
            };

            //println!("------------- iter {}------------\nprev: {:?}\n\nupdate: {:?}", i, prev_row, update_row);
            if let Err(e) = Self::do_wolfram_row(prev_row, update_row, rule) {
                return Err(e);
            }

        }
        Ok(())
    }


    pub fn run_wolfram_rule(mut self, rule: u8, initial_cells: Option<InitialCellConfig>) -> Result<Self, GridError> {
        if let Some(init_cells) = initial_cells {
            for (cell, pos) in init_cells {
                if pos > self.width {
                    return Err(GridError::InitialCellPosIsGreaterThanWidth(format!("Cell cannot be placed outside of grid with, pos: {} width: {}", pos, self.width)))
                }

                let Some(replace_row) = self.board.get_mut(0) else {
                    return Err(GridError::Failure(String::from("Failed to get initial row for grid.")));
                };

                let Some(replace_cell) = replace_row.get_mut(pos) else {
                    return Err(GridError::Failure(String::from("Failed to get replace_cell from row.")))
                };

                replace_cell.state = cell;
            }
        }
        for epoch in 0..self.epochs {
            if let Err(e) = self.do_board_tick(rule) {
                return Err(e)
            };
        }

        Ok(self)
    }
}

impl Grid {
    fn get_next_cell_id(&mut self) -> usize {
        let id = self.cell_count;
        self.cell_count += 1;
        id
    }

    pub fn new(width: usize, epochs: usize) -> Self {
        let mut grid = Self { width, epochs, cell_count: 0, board: Vec::new() };
        for i in 0..epochs {
            let mut row = Vec::new();
            for i in 0..width {
                row.push(Cell::default_grid_cell(i))
            }
            grid.board.push(row);
        }
        grid
    }
}

impl Display for GridError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            GridError::Failure(failure) => {
                f.write_str(failure)
            },
            GridError::InitialCellPosIsGreaterThanWidth(msg) => {
                f.write_str(format!("The initial cell(s) provided are out of the grid bounds: {}", msg).as_str())
            },
            GridError::CouldntWrapNeighborhood(side) => {
                f.write_str(format!("Failed to wrap cell's neighborhood on the {} hand side of the grid", side).as_str())
            }
            GridError::CouldntGetAdjacentCell(side, cell) => {
                f.write_str(format!("Failed to get {} side cell when trying {}", side, cell).as_str())
            }
        }
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