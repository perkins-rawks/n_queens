/*
* Authors: Awildo G., Sam. A., Siqi F., Sosina A.
* Date: May 29, 2020
* Project: Stack
* Desc: We implement a Stack using vectors.
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

        // If the stack is not empty then print in the shape of an array going
        // bottom to top
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