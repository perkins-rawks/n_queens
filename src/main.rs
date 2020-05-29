/*
* Authors: Awildo G., Sam. A., Siqi F., Sosina A.
* Date: May 29, 2020
* Project: N Queens Problem
* Desc: Is it possible to fit N Queen Pieces in an N x N board such that no Queen can be taken?
*/

use std::fmt;

// uses stack.rs in the src "mod" = module
mod my_stack;

// crate is the program (either binary or library, we have binary)
// pub use means 
pub use crate::my_stack::Stack;

// the number of queens on the board, and the size of the board
const N: usize = 8;

pub struct Board {
    // board is an array of booleans
    // true => there is a queen 
    // false => empty space
    board: [[bool; N]; N],
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [[false; N]; N],
        }
    }

    pub fn print(&self) {
        // cannot draw an empty board
        if N == 0 {
            println!("0x0 Chess Board!");
            return;
        }

        // otherwise, draw a pretty board
        print!("+");
        for _i in 0..N {
            print!("---+");
        }
        println!();
        for r in 0..N {
            print!("|");
            for c in 0..N {
                if self.board[r][c] {
                    print!(" Q ");
                } else {
                    print!("   ");
                }
                print!("|");
            }
            println!();
            print!("+");
            for _i in 0..N {
                print!("---+");
            }
            println!();
        }
    }

    // A board should know what a Queen does

    // Is_Safe_Spot determines whether a location is under the line of sight of
    // a Queen
    pub fn safe(&self, _row: usize, _col: usize) -> bool {
        // placeholder
        false
    }

    // Remove Queen
    pub fn remove(&mut self, row: usize, col: usize) {
        self.board[row][col] = false;
    }

    // Add Queen
    pub fn add(&mut self, row: usize, col: usize) {
        self.board[row][col] = true;
    }

}

#[derive(Copy, Clone)]
pub struct Coordinate {
    coords: (usize, usize)
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.coords.0, self.coords.1)
    }
}

// The main algorithm
fn n_queens(board: Board, _stack: Stack<Coordinate>) -> Board {
    // placeholder 
    board
}

fn main() { 
    let mut test1: Board = Board::new();
    test1.add(1, 1);
    test1.print();
    let _b = n_queens(test1, Stack::new());
}
