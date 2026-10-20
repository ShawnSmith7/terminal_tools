use super::{Pos, Canvas, CanvasError, DrawChecked, Draw, Result};

use LineVariant::*;
use CanvasError::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line {
    pub pos: Pos,
    pub len: usize,
    pub ch: char,
    pub variant: LineVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineVariant {
    Horiz,
    Vert,
    DiagDown,
    DiagUp,
}

impl Draw for Line {
    fn draw(&self, canvas: &mut Canvas) {
        match self.variant {
            Horiz => self.draw_horiz(canvas),
            other => {
                let rows = canvas.get_rows_mut(self.pos.row, self.pos.row + self.len);

                match other {
                    Horiz => unreachable!(),
                    Vert => self.draw_vert(rows),
                    DiagDown => self.draw_diag_down(rows),
                    DiagUp => self.draw_diag_up(rows),
                }
            }
        }
    }
}

impl DrawChecked for Line {
    fn draw_checked(&self, canvas: &mut Canvas) -> Result {
        if !self.is_in_bounds(canvas) {
            return Err(OutOfBounds(self.pos));
        }

        self.draw(canvas);
        Ok(())
    }
}

impl Line {
    pub fn new(pos: Pos, len: usize, ch: char, variant: LineVariant) -> Self {
        Self {
            pos,
            len,
            ch,
            variant,
        }
    }

    fn draw_horiz(&self, canvas: &mut Canvas) {
        canvas.get_row_slice_mut(self.pos, self.len).fill(self.ch);
    }

    fn draw_vert<'a>(&self, rows: impl Iterator<Item = &'a mut [char]>) {
        for row in rows {
            unsafe { *row.get_unchecked_mut(self.pos.col) = self.ch; }
        }
    }

    fn draw_diag_down<'a>(&self, rows: impl Iterator<Item = &'a mut [char]>) {
        for (i, row) in rows.enumerate() {
            unsafe { *row.get_unchecked_mut(self.pos.col + 2 * i) = self.ch; }
        }
    }

    fn draw_diag_up<'a>(&self, rows: impl Iterator<Item = &'a mut [char]> + DoubleEndedIterator) {
        for (i, row) in rows.rev().enumerate() {
            unsafe { *row.get_unchecked_mut(self.pos.col + 2 * i) = self.ch; }
        }
    }

    pub fn is_in_bounds(&self, canvas: &Canvas) -> bool {
        if self.len == 0 {
            return true;
        }

        let canvas_size = canvas.size();
        if !canvas.is_in_bounds(self.pos) {
            return false;
        }

        match self.variant {
            Horiz => self.pos.col + self.len <= canvas_size.cols,
            Vert => self.pos.row + self.len <= canvas_size.rows,
            DiagDown => {
                self.pos.row + self.len <= canvas_size.rows
                    && self.pos.col + self.len <= canvas_size.cols
            }
            DiagUp => {
                self.pos.row + self.len <= canvas_size.rows
                    && self.pos.col + self.len <= canvas_size.cols
            }
        }
    }
}