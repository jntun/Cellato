mod grid;
mod cell;
mod rule;

use std::convert::Infallible;
use std::process::{ExitCode, Termination};
use {
    crate::{
        grid::Grid,
        cell::State,
        grid::InitialCellConfig,
    }
};
use crate::CellatoResult::InvalidCommand;
use crate::grid::GridError;

const WIDTH:  usize = 100;
const EPOCHS: usize = 210;
const START_CELL: usize = WIDTH/2;
const RULE: u8 = 1;

/*
 rule: 45
 rule: 50
 rule: 57
 rule: 60

left off:  rule: 88
 */

#[repr(u8)]
pub enum CellatoResult {
    Success = 0,
    InvalidCommand = 64,
    GridError = 70,
}

impl Termination for CellatoResult {
    fn report(self) -> ExitCode {
        ExitCode::from(self as u8)
    }
}

fn main() -> CellatoResult {
    let mut interactive = false;
    let mut epochs: usize     = EPOCHS;
    let mut width:  usize     = WIDTH;
    let mut rule:   u8        = RULE;
    let mut start_config: InitialCellConfig = Vec::new();

    for (argc, arg) in std::env::args().enumerate() {
        match arg.as_str() {
            "-e" => {
                let Some(epoch_arg) = std::env::args().nth(argc+1) else {
                    println!("Internal error processing '-e' flag");
                    return CellatoResult::InvalidCommand
                };
                epochs = epoch_arg.parse::<usize>().unwrap();
            },
            "-w" => {
                let Some(width_arg) = std::env::args().nth(argc+1) else {
                    println!("Internal error processing '-w' flag");
                    return CellatoResult::InvalidCommand
                };
                match width_arg.parse::<u64>() {
                    Ok(width_arg_i) => width = width_arg_i as usize,
                    Err(e) => {
                        println!("Invalid '-w' flag input, want a positive 64-bit integer. Got: {}", width_arg);
                        return InvalidCommand
                    }
                }
            },
            "-r" => {
                let Some(rule_arg) = std::env::args().nth(argc+1) else {
                    println!("Provided '-w' flag without matching value.");
                    return InvalidCommand
                };
                match rule_arg.parse::<u8>() {
                    Ok(rule_arg_u8) => rule = rule_arg_u8,
                    Err(e) => {
                        println!("Invalid '-r' flag input, want a positive 8-bit integer. Got: {}", rule_arg);
                        return InvalidCommand
                    }
                }
            },
            "-i" => interactive = true,
            _ => (),
        }
    }
    start_config.push((State::ON, width/2));
    if interactive {
        if let Err(e) = interactive_grid(epochs, width, rule, start_config) {
            println!("Failed interactive grid: {}", e);
            ncurses::attroff(ncurses::A_BOLD() | ncurses::A_BLINK());
            ncurses::endwin();
            return CellatoResult::GridError
        }
    } else {
        if let Err(e) = generate_static_grid(epochs, width, rule, start_config) {
            println!("Failed to make static grid: {}", e);
            return CellatoResult::GridError
        }
    }
    CellatoResult::Success
}

fn interactive_grid(epochs: usize, width: usize, rule: u8, start_config: InitialCellConfig) -> Result<(), GridError>{
    let mut rule: u8 = rule;

    ncurses::initscr();
    ncurses::raw();

    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::noecho();

    loop {
        ncurses::addstr("\nto advance rule by +1, b to go back -1");
        let ch = ncurses::getch();

        match ch as u32 {
            110 => { rule += 1; if let Err(e) = generate_interactive_grid(epochs, width, rule, start_config.clone()) { return Err(e) }}, // 'n'
            98  => { rule -= 1; if let Err(e) = generate_interactive_grid(epochs, width, rule, start_config.clone()) { return Err(e) }}, // 'b'
            101 => break, // 'e'
            _ => {
                ncurses::attron(ncurses::A_BOLD() | ncurses::A_BLINK());
                ncurses::addstr("\ninvalid key, please re-enter...");
                ncurses::attroff(ncurses::A_BOLD() | ncurses::A_BLINK());
            },
        }
    }

    ncurses::endwin();
    Ok(())
}

fn generate_interactive_grid(epochs: usize, width: usize, rule: u8, start_config: InitialCellConfig) -> Result<(), GridError>{
     match Grid::new(width, epochs).run_wolfram_rule(rule, Some(start_config)) {
         Ok(grid) => {
             ncurses::clear();
             ncurses::attron(ncurses::A_BOLD() | ncurses::A_BLINK());
             ncurses::addstr(format!("\t\t\t\t\trule: {}\n\n", rule).as_str());
             ncurses::addstr(format!("{}\n", grid).as_str());
             ncurses::attroff(ncurses::A_BOLD() | ncurses::A_BLINK());
         },
         Err(e) => return Err(e),
     }
    Ok(())
}

fn generate_static_grid(epochs: usize, width: usize, rule: u8, start_config: InitialCellConfig) -> Result<(), GridError> {
    match Grid::new(width, epochs).run_wolfram_rule(rule, Some(start_config)) {
        Ok(grid) => println!("\t\t\trule: {}\n{}", rule, grid),
        Err(e) => return Err(e),
    }
    Ok(())
}
