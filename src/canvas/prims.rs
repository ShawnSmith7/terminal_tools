use std::ops::Add;
use super::{InBounds, Canvas};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Size {
    pub rows: usize,
    pub cols: usize,
}

impl Size {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

impl Pos {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            row,
            col,
        }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl InBounds for Pos {
    fn is_in_bounds(&self, canvas: &Canvas) -> bool {
        self.row < canvas.size.rows && self.col < canvas.size.cols
    }
    
    fn pos(&self) -> Self {
        *self
    }
}