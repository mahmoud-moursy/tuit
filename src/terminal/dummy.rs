use crate::prelude::Metadata;
use crate::style::Style;
use crate::terminal::{Cell, TerminalConst, TerminalMut};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Default, Hash, Copy, Clone)]
pub struct Dummy;

impl Metadata for Dummy {
    fn dimensions(&self) -> (usize, usize) {
        (0, 0)
    }

    fn default_style(&self) -> Style {
        Style::default()
    }
}

impl TerminalConst for Dummy {
    fn cells(&self) -> impl Iterator<Item=&Cell> {
        [].iter()
    }
}

impl TerminalMut for Dummy {
    fn cells_mut(&mut self) -> impl Iterator<Item=&mut Cell> {
        [].iter_mut()
    }
}