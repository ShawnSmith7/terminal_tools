pub mod line;
mod prims;

pub use prims::{Size, Pos};

use std::fmt::{self, Display, Formatter};
use std::ops::{Index, IndexMut};

use CanvasError::*;

pub type Result<T> = std::result::Result<T, CanvasError>;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Canvas {
    size: Size,
    data: Vec<char>,
}

impl From<Size> for Canvas {
    fn from(size: Size) -> Self {
        Self {
            size,
            data: vec![' '; size.rows * size.cols],
        }
    }
}

impl Display for Canvas {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.size.cols == 0 {
            return Ok(());
        }

        for (i, row) in self.data.chunks(self.size.cols).enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            for &ch in row {
                write!(f, "{ch}")?;
            }
        }

        Ok(())
    }
}

impl Index<Pos> for Canvas {
    type Output = char;

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

    pub fn is_in_bounds(&self, pos: Pos) -> bool {
        pos.row < self.size.rows && pos.col < self.size.cols
    }

    fn index_of_checked(&self, pos: Pos) -> Result<usize> {
        if !self.is_in_bounds(pos) {
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

    pub fn get_checked(&self, pos: Pos) -> Result<char> {
        let idx = self.index_of_checked(pos)?;
        unsafe { Ok(*self.data.get_unchecked(idx)) }
    }

    pub fn get(&self, pos: Pos) -> char {
        let idx = self.index_of(pos);
        unsafe { *self.data.get_unchecked(idx) }
    }

    pub fn get_mut_checked(&mut self, pos: Pos) -> Result<&mut char> {
        let idx = self.index_of_checked(pos)?;
        unsafe { Ok(self.data.get_unchecked_mut(idx)) }
    }

    pub fn get_mut(&mut self, pos: Pos) -> &mut char {
        let idx = self.index_of(pos);
        unsafe { self.data.get_unchecked_mut(idx) }
    }

    pub fn set_checked(&mut self, pos: Pos, ch: char) -> Result<()> {
        *self.get_mut_checked(pos)? = ch;
        Ok(())
    }

    pub fn set(&mut self, pos: Pos, ch: char) {
        *self.get_mut(pos) = ch;
    }

    pub fn get_row_slice_checked(&self, pos: Pos, len: usize) -> Result<&[char]> {
        let idx = self.index_of_checked(pos)?;

        if len > self.size.cols.saturating_sub(pos.col) {
            return Err(OutOfBounds(pos));
        }

        unsafe { Ok(self.data.get_unchecked(idx..idx + len)) }
    }

    pub fn get_row_slice(&self, pos: Pos, len: usize) -> &[char] {
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

    pub fn get_row_slice_mut_checked(&mut self, pos: Pos, len: usize) -> Result<&mut [char]> {
        let idx = self.index_of_checked(pos)?;

        if len > self.size.cols.saturating_sub(pos.col) {
            return Err(OutOfBounds(pos));
        }

        unsafe { Ok(self.data.get_unchecked_mut(idx..idx + len)) }
    }

    pub fn get_row_slice_mut(&mut self, pos: Pos, len: usize) -> &mut [char] {
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
    ) -> Result<impl Iterator<Item = &[char]> + DoubleEndedIterator> {
        if start_row > end_row || end_row > self.size.rows {
            return Err(OutOfBounds(Pos::new(start_row, 0)));
        }

        Ok(self.get_rows_internal(start_row, end_row))
    }

    pub fn get_rows(
        &self,
        start_row: usize,
        end_row: usize,
    ) -> impl Iterator<Item = &[char]> + DoubleEndedIterator {
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
    ) -> impl Iterator<Item = &[char]> + DoubleEndedIterator {
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
    ) -> Result<impl Iterator<Item = &mut [char]> + DoubleEndedIterator> {
        if start_row > end_row || end_row > self.size.rows {
            return Err(OutOfBounds(Pos::new(start_row, 0)));
        }

        Ok(self.get_rows_mut_internal(start_row, end_row))
    }

    pub fn get_rows_mut(
        &mut self,
        start_row: usize,
        end_row: usize,
    ) -> impl Iterator<Item = &mut [char]> + DoubleEndedIterator {
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
    ) -> impl Iterator<Item = &mut [char]> + DoubleEndedIterator {
        let start_idx = start_row * self.size.cols;
        let end_idx = end_row * self.size.cols;

        unsafe {
            self.data
                .get_unchecked_mut(start_idx..end_idx)
                .chunks_exact_mut(self.size.cols)
        }
    }

    pub fn clear(&mut self) {
        self.data.fill(' ');
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

pub trait DrawChecked {
    fn draw_checked(&self, canvas: &mut Canvas) -> Result<()>;
}

pub trait Draw {
    fn draw(&self, canvas: &mut Canvas);
}