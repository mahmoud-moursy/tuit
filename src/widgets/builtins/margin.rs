use crate::Error;
use crate::prelude::{Metadata, Terminal, TerminalConst};
use crate::terminal::{Rectangle, UpdateInfo, UpdateResult, View};
use crate::widgets::{BoundingBox, Widget};

/// Add a [`Margin`] to your widgets! Works by expanding the bounding box, meaning that it may fail
/// to draw in certain cases. Try using [`ShrinkWrap`](crate::widgets::builtins::ShrinkWrap).
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
        let ((rect_left, rect_top), (rect_right, rect_bottom)) = (rect.left_top(), rect.right_bottom());

        let child_left = rect_left.checked_add_signed(self.margin).ok_or(Error::oob())?;
        let child_top = rect_top.checked_add_signed(self.margin).ok_or(Error::oob())?;

        let child_right = rect_right.checked_add_signed(-self.margin).ok_or(Error::oob())?;
        let child_bottom = rect_bottom.checked_add_signed(-self.margin).ok_or(Error::oob())?;

        let child = Rectangle::new((child_left, child_top), (child_right, child_bottom));

        // let child_left = rect_left.checked_add_signed(self.margin).ok_or(Error::OutOfBoundsCoordinate {
        //     x: None,
        //     y: None,
        // })?;
        // let child_top = rect_top.checked_add_signed(self.margin).ok_or(Error::OutOfBoundsCoordinate {
        //     x: None,
        //     y: None,
        // })?;
        //
        // let child = child.at((child_left, child_top));

        let view = View::new(terminal, child).ok_or(Error::OutOfBoundsCoordinate {
            x: Some(child.right()),
            y: Some(child.bottom())
        })?;

        Ok(view)
    }
}

impl<T> BoundingBox for Margin<T>
where T: BoundingBox {
    fn bounding_box(&self, rect: Rectangle) -> crate::Result<Rectangle> {
        let child = self.child.bounding_box(rect)?;

        let Some(offset_child) = child.offset((self.margin, self.margin)) else {
            return Err(Error::oob())
        };

        let Some(fat_child) = offset_child.extend(self.margin) else {
            // Should not fail after we offset the child.
            unreachable!()
        };

        let mut fat_child = fat_child;

        // Negotiate dimensions for the child.
        while !rect.contains_rect(fat_child) {
            if fat_child.right() > rect.right() {
                // Try pushing the fat child into the bounds of the terminal.
                let x_inset = fat_child.right() - rect.right();

                let offset_child = fat_child.offset((-(x_inset as isize), 0));

                if let Some(offset_child) = offset_child {
                    fat_child = offset_child;
                    continue
                }

                #[cfg(feature = "debug")]
                log::trace!("Child is too righteous! Right: {} vs max. {}", fat_child.right(), rect.right());

                #[cfg(feature = "debug")]
                log::trace!("Extending height... from: {}", fat_child.height());
                // Try negotiating more height so that it doesn't need to be as wide.
                fat_child = fat_child.trim_bottom(-1).expect("Should always be valid");
                #[cfg(feature = "debug")]
                log::trace!("NEW HEIGHT: {}", fat_child.height());
                fat_child = fat_child.right_to(rect.right());

                if self.child.bounding_box(fat_child).is_ok() {
                    break
                }
            }

            if fat_child.bottom() > rect.bottom() {
                #[cfg(feature = "debug")]
                log::trace!("Child is too low! Bottom: {} vs max. {}", fat_child.bottom(), rect.bottom());

                // Try pushing the fat child into the bounds of the terminal.
                let y_inset = fat_child.bottom() - rect.bottom();
                let offset_child = fat_child.offset((0, -(y_inset as isize)));

                if let Some(offset_child) = offset_child {
                    fat_child = offset_child;
                    continue
                }

                // Try negotiating more width so that it doesn't need to be as tall.
                fat_child = fat_child.trim_right(-1).expect("Should always be valid");
                fat_child = fat_child.bottom_to(rect.bottom());

                if self.child.bounding_box(fat_child).is_ok() {
                    break
                }
            }

            #[cfg(feature = "debug")]
            log::trace!("Negotiated dimensions of {:?} inside {:?} at {:?}", fat_child.dimensions(), rect.dimensions(), fat_child.left_top());
            #[cfg(feature = "debug")]
            log::trace!("Does this fit? {}", rect.contains_rect(fat_child));

            if fat_child.width() > rect.width()
                && fat_child.height() > rect.height() {
                #[cfg(feature = "debug")]
                log::trace!("Child was too fat at this point.");

                // child is too fat... (cannot fit) :(
                return Err(Error::rescale((fat_child.width(), fat_child.height())))
            }
        }

        #[cfg(feature = "debug")]
        log::trace!("We will be returning {fat_child:?} inside of {rect:?}");


        // let margin_rect = self
        //     .child
        //     .bounding_box(rect)?
        //     .trim_right(-self.margin * 2)
        //     .and_then(|rect| rect.trim_bottom(-self.margin * 2))
        //     .ok_or(Error::RequestRescale {
        //         new_width: rect.width().saturating_add_signed(self.margin),
        //         new_height: rect.height().saturating_add_signed(self.margin),
        //     })?;
        //
        // if !rect.contains_rect(margin_rect) {
        //     return Err(Error::RequestRescale {
        //         new_width: margin_rect.right(),
        //         new_height: margin_rect.bottom(),
        //     })
        // }

        Ok(fat_child)
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