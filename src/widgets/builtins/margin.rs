use crate::Error;
use crate::prelude::{Terminal, TerminalConst};
use crate::terminal::{Rectangle, UpdateInfo, UpdateResult};
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
}

impl<T> BoundingBox for Margin<T>
where T: BoundingBox {
    fn bounding_box(&self, terminal: impl TerminalConst) -> Rectangle {
        terminal
            .bounding_box()
            .extend(-(self.margin as isize))
            .expect("Overflowed due to excessive padding")

    }

    fn completely_covers(&self, _rectangle: Rectangle) -> bool {
        false
    }
}

impl<T> Widget for Margin<T>
where T: BoundingBox {
    fn update(&mut self, update_info: UpdateInfo, terminal: impl TerminalConst) -> crate::Result<UpdateResult> {
        self.child.update(update_info, terminal)
    }

    fn draw(&self, update_info: UpdateInfo, mut terminal: impl Terminal) -> crate::Result<UpdateResult> {
        let rect = self.bounding_box(&terminal);

        let view = terminal.view_mut(rect).ok_or(Error::OutOfBoundsCoordinate {
            x: Some(rect.right()),
            y: Some(rect.bottom())
        })?;

        self.child.draw(update_info, view)
    }
}