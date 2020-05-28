/*
* Authors: Awildo G., Sam. A., Siqi F., Sosina A.
* Date: May 28, 2020
* Project: N Queens Problem
* Desc: Is it possible to fit N Queen Pieces in an N x N board such that no Queen can be taken?
*/

use std::fmt;
use std::marker::Copy; // we want to be able to return copies of objects

// We implement Stack for displayable and copy-able items
pub struct Stack<T: fmt::Display + Copy> {
    // The top of the stack is the last item in contents
    size: usize,
    contents: Vec<T>,
}

impl<T: fmt::Display + Copy> Stack<T> {
    // Constructor of Objects of type Stack.
    pub fn new() -> Self {
        Self {
            size: 0,
            contents: Vec::new(),
        }
    }

    // Returns the top element of the Stack
    pub fn top(&self) -> T {
        assert!(!self.empty());
        self.contents[self.size - 1]
    }

    // Pushes an item onto the top of the Stack
    pub fn push(&mut self, item: T) {
        self.contents.push(item);
        self.size += 1;
    }

    // Pop removes the top of the Stack
    pub fn pop(&mut self) -> T {
        assert!(!self.empty());
        self.size -= 1;

        // deletes the top and returns the top
        self.contents.pop().unwrap()
    }

    // Returns the size of the Stack
    pub fn size(&self) -> usize {
        self.size
    }

    // Returns true when empty, false otherwise
    pub fn empty(&self) -> bool {
        self.size == 0
    }

    // Clears the stack and its contents
    pub fn clear(&mut self) {
        self.contents.clear();
        self.size = 0;
    }
    // Prints the Stack from the bottom to top.
    pub fn print(&self) {
        if self.empty() {
            println!("[]");
            return;
        }

        println!("From bottom to top [Bottom ... Top], the Stack contains: ");
        print!("[");
        for (idx, item) in self.contents.iter().enumerate() {
            if idx < self.size - 1 {
                print!("{}, ", item);
            } else {
                println!("{}]", item);
            }
        }
    }
}

// the number of queens on the board, and the size of the board
const N: usize = 30;

pub struct Board {
    // board is an array of booleans
    // TRUE => YAS QUEEN
    // FALSE => NAH QUEEN
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
