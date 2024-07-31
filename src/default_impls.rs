use core::cmp::Ordering;
use core::ops::{BitOr, Deref, DerefMut};

use crate::prelude::{Metadata, Terminal, TerminalConst, TerminalMut};
use crate::style::{Ansi4, Style};
use crate::terminal::Cell;
use crate::widgets::Rectangle;

impl BitOr for Ansi4 {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        (self as u8) << 4 | rhs as u8
    }
}
impl From<Rectangle> for ((usize, usize), (usize, usize)) {
    fn from(value: Rectangle) -> Self {
        (value.left_top(), value.right_bottom())
    }
}
impl From<((usize, usize), (usize, usize))> for Rectangle {
    fn from((left_top, right_bottom): ((usize, usize), (usize, usize))) -> Self {
        Self::new(left_top, right_bottom)
    }
}

impl PartialOrd for Rectangle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.area().cmp(&other.area()))
    }
}
impl Ord for Rectangle {
    fn cmp(&self, other: &Self) -> Ordering {
        #[allow(clippy::unwrap_used)]
        self.partial_cmp(other).unwrap()
    }
}

impl<T: Deref<Target: Metadata>> Metadata for T {
    fn dimensions(&self) -> (usize, usize) {
        self.deref().dimensions()
    }

    fn default_style(&self) -> Style {
        self.deref().default_style()
    }
}
impl<T: Deref<Target: TerminalConst>> TerminalConst for T {
    // fn dimensions(&self) -> (usize, usize) {
    //     (**self).dimensions()
    // }
    //
    // fn default_style(&self) -> Style {
    //     (**self).default_style()
    // }

    // fn characters_slice_mut(&mut self) -> &mut [Cell] {
    //     (**self).characters_slice_mut()
    // }

    fn characters_slice(&self) -> &[Cell] {
        (**self).characters_slice()
    }
}
impl<T: DerefMut<Target: TerminalMut>> TerminalMut for T {
    fn characters_slice_mut(&mut self) -> &mut [Cell] {
        self.deref_mut().characters_slice_mut()
    }
}
impl<T: TerminalConst + TerminalMut + Metadata> Terminal for T {}