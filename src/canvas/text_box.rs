use super::{Pos, Canvas, Draw, InBounds};
use std::fmt::Alignment;

use Alignment::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextBox<'a> {
    pub pos: Pos,
    pub len: usize,
    pub alignment: Alignment,
    pub text: &'a str,
}

impl<'a> Draw for TextBox<'a> {
    fn draw(&self, canvas: &mut Canvas) {
        let s = match self.alignment {
            Left => format!("{:<1$}", self.text, self.len),
            Right => format!("{:>1$}", self.text, self.len),
            Center => format!("{:^1$}", self.text, self.len),
        };

        for (d, c) in canvas
            .get_row_slice_mut(self.pos, self.len)
            .iter_mut().zip(s.chars()) {
            *d = c;
        }
    }
}

impl<'a> InBounds for TextBox<'a> {
    fn is_in_bounds(&self, canvas: &Canvas) -> bool {
        if self.len == 0 {
            return true;
        }

        if !self.pos.is_in_bounds(canvas) {
            return false;
        }

        self.pos.col + self.len <= canvas.size.cols
    }

    fn pos(&self) -> Pos {
        self.pos
    }
}

impl<'a> TextBox<'a> {
    pub fn new(pos: Pos, len: usize, alignment: Alignment, text: &'a str) -> Self {
        Self {
            pos,
            len,
            alignment,
            text,
        }
    }
}