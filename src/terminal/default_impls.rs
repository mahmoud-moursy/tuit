use core::ops::{Deref, DerefMut};

use crate::prelude::{Metadata, TerminalConst, TerminalMut};
use crate::terminal::{Cell, Style};

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
