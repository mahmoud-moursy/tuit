use crate::prelude::Metadata;
use crate::style::Style;
use crate::terminal::{Cell, TerminalConst, TerminalMut};

#[allow(unused_imports)] // used in docs
use crate::terminal::Terminal;

/// A [`Terminal`] implementation that doesn't do all that much...
///
/// Returns an empty [`Iterator`] when calling [`TerminalConst::cells`] or [`TerminalMut::cells_mut`].
///
/// Dimensions from [`Metadata`] are (0,0), and default style is always the empty style.
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