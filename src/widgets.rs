//! # Widgets
//!
//! The widgets module contains some pre-bundled widgets ready to go with Tuit.

use crate::prelude::*;
use crate::terminal::{MouseButton, TerminalObject, UpdateInfo, UpdateResult};

/// A prompt that is centered
pub struct CenteredPrompt<'a> {
    prompt_text: &'a str,
    // These values are used to calculate where the prompt will be drawn, and are generated on
    // instantiation of the struct.
    height: usize,
    width: usize,
    y_offset: usize,
    x_offset: usize,
    // This value contains the dimensions used to calculate the prompt's cached values at the time of its creation.
    // If the terminal resizes, then the struct will be able to know.
    dimensions_used: (usize, usize),
}

impl<'a> TerminalObject for CenteredPrompt<'a> {
    fn update(&mut self, update_info: UpdateInfo, terminal: impl Terminal) -> crate::Result<UpdateResult> {
        match update_info {
            UpdateInfo::CellClicked(x, y, MouseButton::LeftClick) => {
                let (terminal_width, terminal_height) = terminal.dimensions();

                let index = x + y * terminal_width;

                todo!()
            }
            _ => Ok(UpdateResult::NoEvent)
        }
    }

    fn draw(&mut self, terminal: impl Terminal) {
        todo!()
    }
}

#[cfg(feature = "alloc")]
pub mod alloc {
    //! This module is for widgets that rely upon allocation to function.
    //! The module is only included if the crate's `alloc` feature is enabled!

    extern crate alloc;
}

#[cfg(test)]
#[doc(hidden)]
mod test {}