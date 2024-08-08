use crate::Error;
use crate::prelude::{Metadata, Terminal, TerminalConst};
use crate::terminal::{Rectangle, UpdateInfo, UpdateResult, View};
use crate::widgets::{BoundingBox, Widget};

/// Add padding to your widgets!
pub struct Margin<T> {
    /// The child of the margin.
    child: T,
    /// Padding in all directions
    pub margin: isize
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
    pub const fn margin(mut self, margin: isize) -> Self {
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
        let rect = self.bounding_box_in(&terminal)?;
        let (rect_left, rect_top) = rect.left_top();
        let child = self.child.bounding_box_in(&terminal)?;

        let child_left = rect_left.checked_add_signed(self.margin).ok_or(Error::OutOfBoundsCoordinate {
            x: None,
            y: None,
        })?;
        let child_top = rect_top.checked_add_signed(self.margin).ok_or(Error::OutOfBoundsCoordinate {
            x: None,
            y: None,
        })?;

        let child = child.at((child_left, child_top));

        let view = View::new(terminal, child).ok_or(Error::OutOfBoundsCoordinate {
            x: Some(child.right()),
            y: Some(child.bottom())
        })?;

        Ok(view)
    }
}

impl<T> BoundingBox for Margin<T>
where T: BoundingBox {
    #[allow(clippy::cast_possible_wrap)]
    fn bounding_box(&self, rect: Rectangle) -> crate::Result<Rectangle> {
        let margin_rect = self
            .child
            .bounding_box(rect)?
            .trim_right(-self.margin * 2)
            .and_then(|rect| rect.trim_bottom(-self.margin * 2))
            .ok_or(Error::RequestRescale {
                new_width: rect.width().saturating_add_signed(self.margin),
                new_height: rect.height().saturating_add_signed(self.margin),
            })?;

        if !rect.contains_rect(margin_rect) {
            return Err(Error::RequestRescale {
                new_width: margin_rect.right(),
                new_height: margin_rect.bottom(),
            })
        }

        Ok(margin_rect)
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