use core::ops::{BitOr, Deref, DerefMut};

use crate::prelude::{Metadata, Terminal, TerminalConst, TerminalMut};
use crate::style::{Ansi4, Style};
use crate::terminal::Cell;

impl BitOr for Ansi4 {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        (self as u8) << 4 | rhs as u8
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
