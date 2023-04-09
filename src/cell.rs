use std::fmt::{Debug, Display, Formatter, Write};

#[derive(Copy, Clone)]
pub enum State {
    ON,
    OFF
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ON => f.write_str("on"),
            Self::OFF => f.write_str("off"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Cell {
    id: usize,
    pub state: State,
}

impl Cell {
    pub fn new(id: usize, state: State) -> Self {
        Self {id, state}
    }

    pub fn default_grid_cell(id: usize) -> Self {
        Self {id, state: State::OFF}
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cell")
            .field("#", &self.id)
            .field("state", &self.state)
            .finish()
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.state {
            State::ON => f.write_str("*"),
            State::OFF => f.write_str(" "),
        }
    }
}