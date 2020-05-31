/*
* Authors: Awildo G., Sam. A., Siqi F., Sosina A.
* Date: May 31, 2020
* Project: N Queens Problem
* Desc: Is it possible to fit N Queen Pieces in an N x N board such that no Queen can be taken?
*/

// for display
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
    // Constructor for Board objects
    pub fn new() -> Self {
        Self {
            board: [[false; N]; N],
        }
    }

    // Pretty printing (from Prof. Campbell)
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

    // can_place determines whether it's a safe spot to place a queen assuming
    // there is not a Queen already at (row, col)
    pub fn can_place(&self, row: usize, col: usize) -> bool {
        // if at any point on the horizontal, vertical, and all four diagonal
        // axes there is a queen, then return false

        // If the row and columns are out of bounds, we can't place there.
        if row >= N || col >= N {
            return false;
        }

        // Since the board is a 2d array of booleans, we can check whether there
        // is a queen if the array is true at that point.

        // check the horizontal and vertical
        for idx in 0..N {
            if self.board[row][idx] || self.board[idx][col] {
                return false;
            }
        }

        let mut idx: usize = 0;
        // northwest diagonal
        while (row as i32 - idx as i32) >= 0 && (col as i32 - idx as i32) >= 0 {
            if self.board[row - idx][col - idx] {
                return false;
            }
            idx += 1;
        }

        // northeast diagonal
        idx = 0;
        while (row as i32 - idx as i32) >= 0 && (col + idx < N) {
            if self.board[row - idx][col + idx] {
                return false;
            }
            idx += 1;
        }

        // southwest diagonal
        idx = 0;
        while (row + idx < N) && (col as i32 - idx as i32) >= 0 {
            if self.board[row + idx][col - idx] {
                return false;
            }
            idx += 1;
        }

        // southeast diagonal
        idx = 0;
        while (row + idx < N) && (col + idx < N) {
            if self.board[row + idx][col + idx] {
                return false;
            }
            idx += 1;
        }
        true
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

// Since tuples don't implement display, we make another struct for it.
// Copy and Clone can be derived from simple structs (structs that don't use other
// structs ??).
#[derive(Copy, Clone)]
pub struct Coordinate {
    coords: (usize, usize),
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.coords.0, self.coords.1)
    }
}

// Backtrack will take the most recent move off the stack, and then return the
// most recent
fn backtrack(stack: &mut Stack<Coordinate>) -> Coordinate {
    stack.pop()
}

// The main algorithm
fn n_queens(board: &mut Board, moves: &mut Stack<Coordinate>) {
    // moves is a Stack of locations at which we placed queens (we will use this
    // to backtrack)

    // There aren't any solution for 2 x 2 or 3 x 3 boards
    if N == 2 || N == 3 {
        // Panic stops the program, almost like an error.
        panic!("No solution exists for a {} X {0} chess board.", N);
    }

    let mut row: usize = 0;
    let mut col: usize = 0;

    // There must be queen in each column
    while col < N {
        // Out of bounds
        if row >= N {
            // Backtracking step
            // Get row & col from the top in stack and remove Queen from board
            let action: Coordinate = backtrack(moves);
            row = action.coords.0;
            col = action.coords.1;
            board.remove(row, col);
            row += 1;
        }
        // If we cannot place but are not out of bounds
        else if !board.can_place(row, col) {
            // Just go down one
            row += 1;
        }
        // If we can place
        else {
            // Add the move to the Stack and put a Queen on the board
            // Move on to the next column
            board.add(row, col);
            let action: Coordinate = Coordinate { coords: (row, col) };
            moves.push(action);
            row = 0;
            col += 1;
        }
    }
    board.print();
}

fn main() {
    // Runs the n_queen algorithm on the given board.
    n_queens(&mut Board::new(), &mut Stack::new());
}
