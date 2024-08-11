use crate::Error;
use crate::prelude::Metadata;
use crate::terminal::{Rectangle, Terminal, TerminalConst, UpdateInfo, UpdateResult, View};
use crate::widgets::{BoundingBox, Widget};

/// A widget that shelves two widgets next to each other.
pub struct Shelved<LEFT, RIGHT> {
    /// The widget that is on top.
    pub left_widget: LEFT,
    /// The widget that is on the bottom.
    pub right_widget: RIGHT,
    /// The leftover update from the last [`Widget::update`] call.
    pub leftover_result: Option<UpdateResult>,
}

impl<LEFT, RIGHT> Shelved<LEFT, RIGHT> {
    /// Create a new [`Shelved`] widget.
    pub const fn new(left: LEFT, right: RIGHT) -> Self {
        Self { left_widget: left, right_widget: right, leftover_result: None }
    }

    /// Returns the bounding box of the left widget.
    ///
    /// # Errors
    ///
    /// Will return an error if the bounding box of the right widget is not contained within the bounds of the terminal, or
    /// if the child widget cannot calculate its bounding box.
    pub fn left_view_rect(&self, bounds: Rectangle) -> crate::Result<Rectangle>
    where
        LEFT: BoundingBox,
        RIGHT: BoundingBox {
        let right_view = self.right_widget.bounding_box(bounds)?;
        let left_view = self.left_widget.bounding_box(bounds)?;

        // Lengthen the widget
        let right_lowest = right_view.bottom();
        let left_lowest = left_view.bottom();

        let lowest = right_lowest.max(left_lowest);

        let left_view_widened = left_view.bottom_to(lowest);

        if !bounds.contains_rect(left_view_widened) {
            return Err(Error::RequestRescale {
                new_width: left_view_widened.right(),
                new_height: left_view_widened.bottom(),
            })
        }

        Ok(left_view_widened)
    }

    /// Draws the left widget, and returns its update result. This is better than using [`Widget::draw`]
    /// because it can return a leftover result from the last call to [`Widget::draw`].
    ///
    /// # Errors
    ///
    /// This method will return an error if the left widget's bounding box is larger than the terminal's
    /// bounding box, or if the widget fails to draw.
    pub fn draw_left(&self, update_info: UpdateInfo, mut terminal: impl Terminal) -> crate::Result<UpdateResult>
    where
        LEFT: BoundingBox,
        RIGHT: BoundingBox {
        let left_view = self.get_view_left(&mut terminal)?;

        self.left_widget.draw(update_info, left_view)
    }

    /// Draws the right widget, and returns its update result. This is better than using [`Widget::draw`]
    /// because it can return a leftover result from the last call to [`Widget::draw`].
    ///
    /// # Errors
    ///
    /// This method will return an error if the right widget's bounding box is larger than the terminal's
    /// bounding box, or if the widget fails to draw.
    pub fn draw_right(&self, update_info: UpdateInfo, mut terminal: impl Terminal) -> crate::Result<UpdateResult>
    where
        LEFT: BoundingBox,
        RIGHT: BoundingBox {
        let right_view = self.get_view_right(&mut terminal)?;

        self.right_widget.draw(update_info, right_view)
    }

    /// Draws both widgets, and returns their update results. This is better than using [`Widget::draw`]
    /// because it returns draw update results from both widgets.
    pub fn draw_both(&self, update_info: UpdateInfo, mut terminal: impl Terminal) -> (crate::Result<UpdateResult>, crate::Result<UpdateResult>)
    where
        LEFT: BoundingBox,
        RIGHT: BoundingBox
    {
        let res_left = self.draw_left(update_info, &mut terminal);
        let res_right = self.draw_right(update_info, &mut terminal);

        (res_left, res_right)
    }

    /// Returns the bounding box of the right widget.
    ///
    /// # Errors
    ///
    /// Will return an error if the bounding box of the right widget is not contained within the bounds of the terminal, or
    /// if the child widget cannot calculate its bounding box.
    pub fn right_view_rect(&self, bounds: Rectangle) -> crate::Result<Rectangle>
    where
        LEFT: BoundingBox,
        RIGHT: BoundingBox {

        let right_view = self.right_widget.bounding_box(bounds)?;
        let left_view = self.left_widget.bounding_box(bounds)?;

        // Lengthen the widget
        let right_lowest = right_view.bottom();
        let left_lowest = left_view.bottom();

        let lowest = right_lowest.max(left_lowest);

        let right_view_lengthened = right_view.bottom_to(lowest);

        let right_view_corrected = right_view_lengthened.at(left_view.right_top());

        #[cfg(feature = "debug")]
        log::trace!("Bounding box of right widget: {:?}", right_view_corrected);

        if !bounds.contains_rect(right_view_corrected) {
            #[cfg(feature = "debug")]
            log::trace!("Bounding box of right widget: {:?} BIG OOPSIE", right_view_corrected);
            return Err(Error::rescale_to(right_view_corrected));
        }

        Ok(right_view_corrected)
    }

    /// Returns a [`View`] of the right widget.
    ///
    /// # Errors
    ///
    /// Will return an error if the bounding box of the right widget is not contained within the bounds of the terminal, or
    /// if the child widget cannot calculate its bounding box.
    pub fn get_view_right<T: Metadata>(&self, terminal: T) -> crate::Result<View<T>>
    where
        LEFT: BoundingBox,
        RIGHT: BoundingBox {
        let right_view_rect = self.right_view_rect(terminal.bounding_box())?;
        View::new(terminal, right_view_rect).ok_or_else(|| Error::rescale_to(right_view_rect))
    }

    /// Returns a [`View`] of the left widget.
    ///
    /// # Errors
    ///
    /// Will return an error if the bounding box of the right widget is not contained within the bounds of the terminal, or
    /// if the child widget cannot calculate its bounding box.
    pub fn get_view_left<T: Metadata>(&self, terminal: T) -> crate::Result<View<T>>
    where
        LEFT: BoundingBox,
        RIGHT: BoundingBox {
        let left_view_rect = self.left_view_rect(terminal.bounding_box())?;
        View::new(terminal, left_view_rect).ok_or_else(|| Error::rescale_to(left_view_rect))
    }
}

impl<LEFT: BoundingBox, RIGHT: BoundingBox> Widget for Shelved<LEFT, RIGHT> {
    fn update(&mut self, update_info: UpdateInfo, terminal: impl TerminalConst) -> crate::Result<UpdateResult> {
        let left_view = self.get_view_left(&terminal)?;
        let left_update =  self.left_widget.update(update_info, left_view)?;

        let right_view = self.get_view_right(&terminal)?;
        let right_update = self.right_widget.update(update_info, right_view)?;

        self.leftover_result = Some(left_update.min(right_update));

        Ok(left_update.max(right_update))
    }

    fn draw(&self, update_info: UpdateInfo, mut terminal: impl Terminal) -> crate::Result<UpdateResult> {
        let left_view = self.get_view_left(&mut terminal)?;
        let left_update = self.left_widget.draw(update_info, left_view)?;

        let right_view = self.get_view_right(&mut terminal)?;

        let right_update = self.right_widget.draw(update_info, right_view)?;

        Ok(left_update.max(right_update))
    }
}

impl<LEFT: BoundingBox, RIGHT: BoundingBox> BoundingBox for Shelved<LEFT, RIGHT> {
    fn bounding_box(&self, terminal_rect: Rectangle) -> crate::Result<Rectangle> {
        let left_height = self.left_widget.bounding_box(terminal_rect)?.height();
        let right_height = self.right_widget.bounding_box(terminal_rect)?.height();

        let left_width = self.left_widget.bounding_box(terminal_rect)?.width();
        let right_width = self.right_widget.bounding_box(terminal_rect)?.width();

        let width = left_width + right_width;
        let height = left_height.max(right_height);

        let left_top = self.left_widget.bounding_box(terminal_rect)?.left_top();

        let rect = Rectangle::of_size((width, height)).at(left_top);

        if !terminal_rect.contains_rect(rect) {
            return Err(Error::rescale_to(rect));
        }

        Ok(rect)
    }

    fn completely_covers(&self, bounds: Rectangle) -> bool {
        let Ok(rect) = self.bounding_box(bounds) else { return false };

        let Ok(left_view_rect) = self.left_view_rect(bounds) else { return false };

        let Ok(right_view_rect) = self.right_view_rect(bounds) else { return false };

        rect.dimensions() == bounds.dimensions()
            && self.left_widget.completely_covers(left_view_rect)
            && self.right_widget.completely_covers(right_view_rect)
    }
}