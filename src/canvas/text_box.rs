use super::{Pos, Canvas, Draw, InBounds};
use std::fmt::Alignment;
use colored::*;

use Alignment::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextBox {
    pub pos: Pos,
    pub len: usize,
    pub alignment: Alignment,
    pub text: ColoredString,
}

impl Draw for TextBox {
    fn draw(&self, canvas: &mut Canvas) {
        let plain_text = strip_ansi(&self.text);
        let visible_len = plain_text.chars().count();

        let pad = self.len.saturating_sub(visible_len);
        let (left_pad, right_pad) = match self.alignment {
            Left => (0, pad),
            Right => (pad, 0),
            Center => (pad / 2, pad - (pad / 2)),
        };

        let fg = self.text.fgcolor;
        let bg = self.text.bgcolor;
        let style = self.text.style;

        let left_stream = std::iter::repeat(" ".normal()).take(left_pad);

        let text_stream = plain_text.chars().map(|ch| {
            let mut colored_char = ch.to_string().normal();
            if let Some(color) = fg { colored_char = colored_char.color(color); }
            if let Some(color) = bg { colored_char = colored_char.on_color(color); }
            colored_char.style = style;
            colored_char
        });

        let right_stream = std::iter::repeat(" ".normal()).take(right_pad);

        let cell_stream = left_stream.chain(text_stream).chain(right_stream);

        for (d, cell) in canvas
            .get_row_slice_mut(self.pos, self.len)
            .iter_mut()
            .zip(cell_stream)
        {
            *d = cell;
        }
    }
}

impl InBounds for TextBox {
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

impl TextBox {
    pub fn new(pos: Pos, len: usize, alignment: Alignment, text: ColoredString) -> Self {
        Self {
            pos,
            len,
            alignment,
            text,
        }
    }
}

fn strip_ansi(s: &str) -> String {
    let mut in_escape = false;
    s.chars()
        .filter(|&c| {
            if c == '\x1b' {
                in_escape = true;
                false
            } else if in_escape {
                if c.is_ascii_alphabetic() || c == 'm' {
                    in_escape = false;
                }
                false
            } else {
                true
            }
        })
        .collect()
}