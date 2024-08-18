use crate::prelude::{Terminal, TerminalConst};
use crate::style::{Colour, Style};
use crate::terminal::{UpdateInfo, UpdateResult, View};
use crate::widgets::builtins::Sweeper;
use crate::widgets::{BoundingBox, Widget};
use crate::Error;

/// A widget that draws a backdrop behind its children.
#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct Backdrop<'a, T> {
    /// The style of the backdrop.
    pub style: Style,
    child: &'a T
}

impl<'a, T> Backdrop<'a, T> {
    /// Create a new [`Backdrop`] with the specified [`Style`].
    #[must_use]
    pub const fn new(child: &'a T) -> Self {
        Self { style: Style::new(), child }
    }

    /// Create a new [`Backdrop`] with the specified [`Colour`].
    #[must_use]
    pub const fn of_colour(mut self, colour: Colour) -> Self {
        self.style = self.style.bg(colour);

        self
    }

    /// Set the [`Style`] of the [`Backdrop`] and return it.
    #[must_use]
    pub const fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a, T> Widget for Backdrop<'a, T>
where T: BoundingBox{
    fn update(&mut self, _update_info: UpdateInfo, _terminal: impl TerminalConst) -> crate::Result<UpdateResult> {
        Ok(UpdateResult::NoEvent)
    }

    fn draw(&self, terminal: impl Terminal) -> crate::Result<UpdateResult> {
        let child_bounding_box = self.child.bounding_box_in(&terminal)?;
        let mut view = View::new(terminal, child_bounding_box).ok_or(Error::oob())?;
        let sweeper = Sweeper::new(self.style);

        sweeper.drawn(&mut view)?;

        Ok(UpdateResult::NoEvent)
    }
}