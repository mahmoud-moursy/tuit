use crate::Error;
use crate::prelude::{Metadata, Terminal, TerminalConst};
use crate::terminal::{Rectangle, UpdateInfo, UpdateResult, View};
use crate::widgets::{BoundingBox, Widget};

/// Add padding to your widgets!
pub struct Margin<T> {
    /// The child of the margin.
    child: T,
    /// Padding in all directions
    pub margin: usize
}

impl<T> Margin<T> {
    /// Make a new [`Margin`]
    #[must_use]
    pub const fn new(child: T) -> Self {
        Self {
            child,
            margin: 0
        }
    }

    /// Give the [`Margin`] a fixed amount of padding
    #[must_use]
    pub const fn margin(mut self, margin: usize) -> Self {
        self.margin = margin;

        self
    }

    /// Get the inner value of the [`Margin`]
    pub fn into_inner(self) -> T {
        self.child
    }

    /// Get a reference to the inner value of the [`Margin`].
    pub const fn inner(&self) -> &T {
        &self.child
    }

    fn margin_view<U: Metadata>(&self, terminal: U) -> crate::Result<View<U>>
    where T: BoundingBox {
        let rect = self.bounding_box(&terminal)?;

        let view = View::new(terminal, rect).ok_or(Error::OutOfBoundsCoordinate {
            x: Some(rect.right()),
            y: Some(rect.bottom())
        })?;

        Ok(view)
    }
}

impl<T> BoundingBox for Margin<T>
where T: BoundingBox {
    #[allow(clippy::cast_possible_wrap)]
    fn bounding_box(&self, terminal: impl Metadata) -> crate::Result<Rectangle> {
        terminal
            .bounding_box()
            .extend(-(self.margin as isize))
            .ok_or(Error::OutOfBoundsCoordinate {
                x: None,
                y: None,
            })

    }

    // The margin does not draw over the surrounding space,
    // so it does not completely cover the [`Rectangle`].
    fn completely_covers(&self, _rectangle: Rectangle) -> bool {
        false
    }
}

impl<T> Widget for Margin<T>
where T: BoundingBox {
    fn update(&mut self, update_info: UpdateInfo, terminal: impl TerminalConst) -> crate::Result<UpdateResult> {
        let view = self.margin_view(terminal)?;

        self.child.update(update_info, view)
    }

    fn draw(&self, update_info: UpdateInfo, terminal: impl Terminal) -> crate::Result<UpdateResult> {
        let view = self.margin_view(terminal)?;

        self.child.draw(update_info, view)
    }
}