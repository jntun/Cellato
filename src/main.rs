mod grid;
mod cell;
mod rule;

use {
    crate::{
        grid::Grid,
        cell::State,
        grid::InitialCellConfig,
    }
};

const WIDTH:  usize = 500;
const EPOCHS: usize = 500;
const START_CELL: usize = WIDTH/2;

/*
 rule: 45
 rule: 50
 rule: 57
 rule: 60

left off:  rule: 88
 */

fn main() {
    let mut rule: u8= 0;
    ncurses::initscr();
    ncurses::raw();

    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::noecho();


    loop {
        ncurses::addstr("\nto advance rule by +1, b to go back -1");
        let ch = ncurses::getch();

        match ch as u32 {
            110 => { rule += 1; generate_grid(EPOCHS, WIDTH, rule) }, // 'n'
            98  => { rule -= 1; generate_grid(EPOCHS, WIDTH, rule) }, // 'b'
            101 => break, // 'e'
            _ => {
                ncurses::attron(ncurses::A_BOLD() | ncurses::A_BLINK());
                ncurses::addstr("\ninvalid key, please re-enter...");
                ncurses::attroff(ncurses::A_BOLD() | ncurses::A_BLINK());
            },
        }
    }

    ncurses::endwin();
}

fn generate_grid(epochs: usize, width: usize, rule: u8)  {
     match Grid::new(width, epochs).run_wolfram_rule(rule, Some(vec![(State::ON, START_CELL)])) {
         Ok(grid) => {
             ncurses::clear();
             ncurses::attron(ncurses::A_BOLD() | ncurses::A_BLINK());
             ncurses::addstr(format!("\t\t\t\t\trule: {}\n\n", rule).as_str());
             ncurses::addstr(format!("{}\n", grid).as_str());
             ncurses::attroff(ncurses::A_BOLD() | ncurses::A_BLINK());
         },
         Err(e) => println!("Failed rule {}: {}", rule, e),
     }
}
