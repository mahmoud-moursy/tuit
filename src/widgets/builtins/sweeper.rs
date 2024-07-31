use crate::prelude::{Terminal, TerminalConst, Widget};
use crate::style::Colour;
use crate::terminal::{UpdateInfo, UpdateResult};

/// A widget that will clear the entire terminal and replace it with a blank cell containing
/// the specified colour.
pub struct Sweeper {
    /// The colour to use for the blank cells that clear the terminal.
    pub colour: Colour,
}

impl Sweeper {
    /// Creates a new [`Sweeper`] with the specified `colour`
    #[must_use]
    pub const fn new(colour: Colour) -> Self {
        Self { colour }
    }
}

impl Widget for Sweeper {
    fn update(
        &mut self,
        _update_info: UpdateInfo,
        _terminal: impl TerminalConst,
    ) -> crate::Result<UpdateResult> {
        Ok(UpdateResult::NoEvent)
    }

    fn draw(
        &self,
        _update_info: UpdateInfo,
        mut terminal: impl Terminal,
    ) -> crate::Result<UpdateResult> {
        for character in terminal.characters_slice_mut() {
            character.style.bg_colour = Some(self.colour);
            character.character = ' ';
        }

        Ok(UpdateResult::NoEvent)
    }
}