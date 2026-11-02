pub mod line;
mod prims;
mod text_box;

pub use prims::{Size, Pos};
pub use text_box::TextBox;

use std::{fmt, ops, result};
use fmt::{Display, Formatter, Alignment};
use ops::{Index, IndexMut};
use terminal_size::{terminal_size, Width};
use colored::*;

use CanvasError::*;

pub type Result<T = ()> = result::Result<T, CanvasError>;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Canvas {
    size: Size,
    data: Vec<ColoredString>,
}

impl From<Size> for Canvas {
    fn from(size: Size) -> Self {
        Self {
            size,
            data: vec![" ".normal(); size.rows * size.cols],
        }
    }
}

impl Display for Canvas {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.size.cols == 0 {
            return Ok(());
        }

        let target_width = f.width().unwrap_or(self.size.cols);

        let total_pad = target_width.saturating_sub(self.size.cols);

        if total_pad == 0 {
            for (i, row) in self.data.chunks(self.size.cols).enumerate() {
                if i > 0 {
                    writeln!(f)?;
                }
                for ch in row {
                    write!(f, "{ch}")?;
                }
            }
            return Ok(());
        }

        let fill = f.fill();
        let alignment = f.align().unwrap_or(Alignment::Left);

        let (left_pad, right_pad) = match alignment {
            Alignment::Left => (0, total_pad),
            Alignment::Center => (total_pad / 2, total_pad - (total_pad / 2)),
            Alignment::Right => (total_pad, 0),
        };

        let left_str = fill.to_string().repeat(left_pad);
        let right_str = fill.to_string().repeat(right_pad);

        for (i, row) in self.data.chunks(self.size.cols).enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{left_str}")?;
            for ch in row {
                write!(f, "{ch}")?;
            }
            write!(f, "{right_str}")?;
        }

        Ok(())
    }
}

impl Index<Pos> for Canvas {
    type Output = ColoredString;

    fn index(&self, pos: Pos) -> &Self::Output {
        let idx = self.index_of(pos);
        unsafe { self.data.get_unchecked(idx) }
    }
}

impl IndexMut<Pos> for Canvas {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        let idx = self.index_of(pos);
        unsafe { self.data.get_unchecked_mut(idx) }
    }
}

impl Canvas {
    pub fn size(&self) -> Size {
        self.size
    }

    fn index_of_checked(&self, pos: Pos) -> Result<usize> {
        if !pos.is_in_bounds(self) {
            return Err(OutOfBounds(pos));
        }

        Ok(self.index_of_internal(pos))
    }

    fn index_of(&self, pos: Pos) -> usize {
        #[cfg(debug_assertions)]
        { self.index_of_checked(pos).unwrap() }
        #[cfg(not(debug_assertions))]
        { self.index_of_internal(pos) }
    }

    #[inline(always)]
    fn index_of_internal(&self, pos: Pos) -> usize {
        pos.row * self.size.cols + pos.col
    }

    pub fn get_checked(&self, pos: Pos) -> Result<&ColoredString> {
        let idx = self.index_of_checked(pos)?;
        unsafe { Ok(self.data.get_unchecked(idx)) }
    }

    pub fn get(&self, pos: Pos) -> &ColoredString {
        let idx = self.index_of(pos);
        unsafe { &self.data.get_unchecked(idx) }
    }

    pub fn get_mut_checked(&mut self, pos: Pos) -> Result<&mut ColoredString> {
        let idx = self.index_of_checked(pos)?;
        unsafe { Ok(self.data.get_unchecked_mut(idx)) }
    }

    pub fn get_mut(&mut self, pos: Pos) -> &mut ColoredString {
        let idx = self.index_of(pos);
        unsafe { self.data.get_unchecked_mut(idx) }
    }

    pub fn set_checked(&mut self, pos: Pos, ch: ColoredString) -> Result {
        *self.get_mut_checked(pos)? = ch;
        Ok(())
    }

    pub fn set(&mut self, pos: Pos, ch: ColoredString) {
        *self.get_mut(pos) = ch;
    }

    pub fn get_row_slice_checked(&self, pos: Pos, len: usize) -> Result<&[ColoredString]> {
        let idx = self.index_of_checked(pos)?;

        if len > self.size.cols.saturating_sub(pos.col) {
            return Err(OutOfBounds(pos));
        }

        unsafe { Ok(self.data.get_unchecked(idx..idx + len)) }
    }

    pub fn get_row_slice(&self, pos: Pos, len: usize) -> &[ColoredString] {
        #[cfg(debug_assertions)]
        {
            self.get_row_slice_checked(pos, len).unwrap()
        }
        #[cfg(not(debug_assertions))]
        {
            let idx = self.index_of(pos);
            unsafe { &self.data.get_unchecked(idx..idx + len) }
        }
    }

    pub fn get_row_slice_mut_checked(&mut self, pos: Pos, len: usize) -> Result<&mut [ColoredString]> {
        let idx = self.index_of_checked(pos)?;

        if len > self.size.cols.saturating_sub(pos.col) {
            return Err(OutOfBounds(pos));
        }

        unsafe { Ok(self.data.get_unchecked_mut(idx..idx + len)) }
    }

    pub fn get_row_slice_mut(&mut self, pos: Pos, len: usize) -> &mut [ColoredString] {
        #[cfg(debug_assertions)]
        {
            self.get_row_slice_mut_checked(pos, len).unwrap()
        }
        #[cfg(not(debug_assertions))]
        {
            let idx = self.index_of(pos);
            unsafe { self.data.get_unchecked_mut(idx..idx + len) }
        }
    }

    pub fn get_rows_checked(
        &self,
        start_row: usize,
        end_row: usize,
    ) -> Result<impl Iterator<Item = &[ColoredString]> + DoubleEndedIterator> {
        if start_row > end_row || end_row > self.size.rows {
            return Err(OutOfBounds(Pos::new(start_row, 0)));
        }

        Ok(self.get_rows_internal(start_row, end_row))
    }

    pub fn get_rows(
        &self,
        start_row: usize,
        end_row: usize,
    ) -> impl Iterator<Item = &[ColoredString]> + DoubleEndedIterator {
        #[cfg(debug_assertions)]
        {
            self.get_rows_checked(start_row, end_row).unwrap()
        }
        #[cfg(not(debug_assertions))]
        {
            self.get_rows_internal(start_row, end_row)
        }
    }

    #[inline(always)]
    fn get_rows_internal(
        &self,
        start_row: usize,
        end_row: usize,
    ) -> impl Iterator<Item = &[ColoredString]> + DoubleEndedIterator {
        let start_idx = start_row * self.size.cols;
        let end_idx = end_row * self.size.cols;

        unsafe {
            self.data
                .get_unchecked(start_idx..end_idx)
                .chunks_exact(self.size.cols)
        }
    }

    pub fn get_rows_mut_checked(
        &mut self,
        start_row: usize,
        end_row: usize
    ) -> Result<impl Iterator<Item = &mut [ColoredString]> + DoubleEndedIterator> {
        if start_row > end_row || end_row > self.size.rows {
            return Err(OutOfBounds(Pos::new(start_row, 0)));
        }

        Ok(self.get_rows_mut_internal(start_row, end_row))
    }

    pub fn get_rows_mut(
        &mut self,
        start_row: usize,
        end_row: usize,
    ) -> impl Iterator<Item = &mut [ColoredString]> + DoubleEndedIterator {
        #[cfg(debug_assertions)]
        {
            self.get_rows_mut_checked(start_row, end_row).unwrap()
        }
        #[cfg(not(debug_assertions))]
        {
            self.get_rows_mut_internal(start_row, end_row)
        }
    }

    #[inline(always)]
    fn get_rows_mut_internal(
        &mut self,
        start_row: usize,
        end_row: usize,
    ) -> impl Iterator<Item = &mut [ColoredString]> + DoubleEndedIterator {
        let start_idx = start_row * self.size.cols;
        let end_idx = end_row * self.size.cols;

        unsafe {
            self.data
                .get_unchecked_mut(start_idx..end_idx)
                .chunks_exact_mut(self.size.cols)
        }
    }

    pub fn clear(&mut self) {
        self.data.fill(" ".normal());
    }

    pub fn print_centered(&self) {
        let term_width = match terminal_size() {
            Some((Width(w), _)) => w as usize,
            None => self.size.cols,
        };

        println!("{:^1$}", self, term_width);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasError {
    OutOfBounds(Pos),
}

impl Display for CanvasError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            OutOfBounds(pos) => write!(f, "{pos:?} is out of mod bounds"),
        }
    }
}

impl std::error::Error for CanvasError {}

pub trait Draw {
    fn draw(&self, canvas: &mut Canvas);
}

pub trait InBounds {
    fn is_in_bounds(&self, canvas: &Canvas) -> bool;
    fn pos(&self) -> Pos;
}

pub trait DrawChecked {
    fn draw_checked(&self, canvas: &mut Canvas) -> Result;
}

impl<T: Draw + InBounds> DrawChecked for T {
    fn draw_checked(&self, canvas: &mut Canvas) -> Result {
        if !self.is_in_bounds(canvas) {
            return Err(OutOfBounds(self.pos()));
        }

        self.draw(canvas);
        Ok(())
    }
}