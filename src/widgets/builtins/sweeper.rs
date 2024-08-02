use crate::prelude::{Terminal, TerminalConst, Widget};
use crate::style::{Colour, Style};
use crate::terminal::{UpdateInfo, UpdateResult};

/// A widget that will clear the entire terminal and replace it with a blank cell containing
/// the specified style.
pub struct Sweeper {
    /// The style to use for the blank cells that clear the terminal.
    pub style: Style,
}

impl Sweeper {
    /// Creates a new [`Sweeper`] with the specified [`Style`]
    #[must_use]
    pub const fn new(style: Style) -> Self {
        Self { style }
    }

    /// Creates a new [`Sweeper`] with the specified [`Colour`] and a blank style
    #[must_use]
    pub const fn of_colour(colour: Colour) -> Self {
        Self {
            style: Style::new().bg(colour)
        }
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
        for character in terminal.cells_mut() {
            character.style = self.style;
            character.character = ' ';
        }

        Ok(UpdateResult::NoEvent)
    }
}