use crate::color::Color;
use std::fmt;
use termion::color;

#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub chr: char,
    pub color: Color,
}

impl Block {
    pub fn new(chr: char, color: Color) -> Self {
        Block { chr, color }
    }

    pub fn render_next(&self) -> String {
        format!(
            "{}{}{}",
            color::Fg(self.color),
            self.chr,
            color::Fg(color::Black),
        )
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", color::Bg(self.color), color::Bg(color::Black))
    }
}
