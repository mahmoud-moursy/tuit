use crate::Error::RequestRescale;
use crate::prelude::{ Terminal, TerminalConst, Widget};
use crate::terminal::{Rectangle, UpdateInfo, UpdateResult};
use crate::widgets::BoundingBox;

/// A widget that centers its child widget within its bounding box.
///
/// Child widgets need to implement [`BoundingBox`].
pub struct Centered<T> {
    child: T,
    centered_x: bool,
    centered_y: bool,
}

impl<T> Centered<T> {
    /// Make a new [`Centered`] widget.
    pub const fn new(child: T) -> Self {
        Self { child, centered_x: true, centered_y: true }
    }

    /// Consume the [`Centered`] widget and return the inner widget.
    #[must_use]
    pub fn into_inner(self) -> T {
        self.child
    }

    /// Get a reference to the inner widget.
    #[must_use]
    pub const fn inner(&self) -> &T {
        &self.child
    }
}

impl<T: BoundingBox> Widget for Centered<T> {
    fn update(&mut self, update_info: UpdateInfo, terminal: impl TerminalConst) -> crate::Result<UpdateResult> {
        let bounding_box = self.bounding_box_in(&terminal)?;
        let view = terminal.view(bounding_box).ok_or(RequestRescale {
            new_width: bounding_box.right(),
            new_height: bounding_box.bottom(),
        })?;

        self.child.update(update_info, view)
    }

    fn draw(&self, update_info: UpdateInfo, mut terminal: impl Terminal) -> crate::Result<UpdateResult> {
        let bounding_box = self.bounding_box_in(&terminal)?;
        let view = terminal.view_mut(bounding_box).ok_or(RequestRescale {
            new_width: bounding_box.right(),
            new_height: bounding_box.bottom(),
        })?;

        self.child.draw(update_info, view)
    }
}

impl<T: BoundingBox> BoundingBox for Centered<T> {
    fn bounding_box(&self, rect: Rectangle) -> crate::Result<Rectangle> {
        let (terminal_width, terminal_height) = rect.dimensions();
        let (widget_width, widget_height) = self.child.bounding_box(rect)?.dimensions();

        let horizontal_center = terminal_width / 2;
        let vertical_center = terminal_height / 2;

        let left = if self.centered_x {
            horizontal_center - (widget_width / 2)
        } else {
            rect.left()
        };
        let right = left + widget_width;


        let top = if self.centered_y {
            vertical_center -(widget_height / 2)
        } else {
            rect.top()
        };
        let bottom = top + widget_height;

        Ok(Rectangle::new((left, top), (right, bottom)))
    }

    fn completely_covers(&self, rectangle: Rectangle) -> bool {
        self.child.completely_covers(rectangle)
    }
}