//! # Widgets
//!
//! The widgets module contains some pre-bundled widgets ready to go with Tuit.
//! 
//! Heavy TODO!

use crate::prelude::*;
use crate::terminal::{MouseButton, TerminalColour, TerminalObject, UpdateInfo, UpdateResult};

/// A widget that will clear the entire terminal and replace it with a blank cell containing
/// the specified colour.
pub struct Sweeper {
    /// The colour to use for the blank cells that clear the terminal.
    pub colour: TerminalColour
}

impl Sweeper {
    /// Creates a new [`Sweeper`] with the specified `colour`
    pub fn new(colour: TerminalColour) -> Self {
        Sweeper {
            colour
        }
    }
}

impl TerminalObject for Sweeper {
    fn update(&mut self, _update_info: UpdateInfo, _terminal: impl Terminal) -> crate::Result<UpdateResult> {
        Ok(UpdateResult::NoEvent)
    }

    fn draw(&mut self, _update_info: UpdateInfo, mut terminal: impl Terminal) -> crate::Result<UpdateResult> {
        for character in terminal.characters_mut() {
            character.style.bg_colour = Some(self.colour);
            character.character = ' ';
        }

        Ok(UpdateResult::NoEvent)
    }
}

/// A prompt that is centered
pub struct CenteredPrompt<'a> {
    /// The text to be displayed
    prompt_text: &'a str,
    // These values are used to calculate where the prompt will be drawn, and are generated on
    // instantiation of the struct.
    height: usize,
    width: usize,
    y_offset: usize,
    x_offset: usize,
    // This value contains the dimensions used to calculate the prompt's cached values at the time of its creation.
    // If the terminal resizes, then the struct will be able to know.
    dimensions_used: Option<(usize, usize)>,
}

impl<'a> TerminalObject for CenteredPrompt<'a> {
    fn update(&mut self, update_info: UpdateInfo, _terminal: impl Terminal) -> crate::Result<UpdateResult> {
        match update_info {
            UpdateInfo::CellClicked(x, y, MouseButton::LeftClick) => {
                #[allow(clippy::collapsible_if)]
                // Check if click was within bounds.
                if x < self.width + self.x_offset && self.x_offset > x {
                    if y < self.height + self.y_offset && self.y_offset > y {
                        return Ok(UpdateResult::LifecycleEnd)
                    }
                }

                Ok(UpdateResult::NoEvent)
            }
            _ => Ok(UpdateResult::NoEvent)
        }
    }

    fn draw(&mut self, update_info: UpdateInfo, terminal: impl Terminal) -> crate::Result<UpdateResult> {
        let dimensions = terminal.dimensions();

        #[allow(clippy::collapsible_if)]
        if update_info != UpdateInfo::ForceRedraw {
            if self.dimensions_used == Some(dimensions) {
                return Ok(UpdateResult::NoRedraw);
            }
        }



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