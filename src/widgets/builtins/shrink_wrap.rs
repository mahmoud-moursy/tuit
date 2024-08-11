use crate::Error;
use crate::prelude::{Terminal, TerminalConst, Widget};
use crate::terminal::{Metadata, Rectangle, UpdateInfo, UpdateResult, View};
use crate::widgets::BoundingBox;

/// Adds padding to a [`BoundingBox`](crate::widgets::BoundingBox) widget by pretending that the terminal is smaller than it actually is
pub struct ShrinkWrap<T> {
    /// The child widget
    child: T,
    /// How much padding to apply
    pub padding: usize
}

impl<T> ShrinkWrap<T> {

    /// Create a new [`ShrinkWrap`] with a child widget. Use [`ShrinkWrap::shrink`] to set padding.
    /// You can also set the `padding` field directly on the [`ShrinkWrap`].
    #[must_use]
    pub const fn new(child: T) -> Self {
        Self {
            child,
            padding: 0
        }
    }

    /// Set the padding for the widget.
    #[must_use]
    pub const fn shrink(mut self, padding: usize) -> Self {
        self.padding = padding;

        self
    }

    /// Get a reference to the child widget
    pub const fn inner(&self) -> &T {
        &self.child
    }

    /// Returns the
    ///
    /// # Errors
    ///
    /// Will return an error if the [`ShrinkWrap`] can't fit its bounding box.
    pub fn get_inner_view<U>(&self, terminal: U) -> crate::Result<View<U>>
    where
          T: Widget,
          U: Metadata {
        let bounding_box = self.bounding_box_in(&terminal)?;
        let view = View::new(terminal, bounding_box);
        let view_with_err = view.ok_or(Error::RequestRescale { new_width: bounding_box.right(), new_height: bounding_box.bottom() });

        view_with_err
    }

    /// Consume [`self`] and return the child widget.
    pub fn into_inner(self) -> T {
        self.child
    }
}

impl<T: Widget> Widget for ShrinkWrap<T> {
    fn update(&mut self, update_info: UpdateInfo, terminal: impl TerminalConst) -> crate::Result<UpdateResult> {
        let view = self.get_inner_view(terminal)?;

        self.child.update(update_info, view)
    }

    fn draw(&self, update_info: UpdateInfo, terminal: impl Terminal) -> crate::Result<UpdateResult> {
        let view = self.get_inner_view(terminal)?;

        self.child.draw(update_info, view)
    }
}

impl<T: Widget> BoundingBox for ShrinkWrap<T> {
    fn bounding_box(&self, rect: Rectangle) -> crate::Result<Rectangle> {
        rect
            .trim_x(self.padding as isize)
            .and_then(|rect| rect.trim_y(self.padding as isize))
            .ok_or(Error::RequestRescale {
                new_width: rect.width().saturating_add_signed(self.padding as isize),
                new_height: rect.height().saturating_add_signed(self.padding as isize),
            })
    }

    fn completely_covers(&self, _rectangle: Rectangle) -> bool {
        false
    }
}