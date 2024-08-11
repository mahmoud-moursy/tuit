use crate::style::{Colour, Style};

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug, Default)]
pub struct Backdrop {
    pub style: Style,
}

impl Backdrop {
    /// Create a new [`Backdrop`] with the specified [`Style`].
    #[must_use]
    pub const fn new(style: Style) -> Self {
        Self { style }
    }

    /// Create a new [`Backdrop`] with the specified [`Colour`].
    #[must_use]
    pub const fn of_colour(colour: Colour) -> Self {
        Self::new(Style::new().bg(colour))
    }
}