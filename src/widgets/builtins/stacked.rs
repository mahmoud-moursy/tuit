use crate::Error;
use crate::prelude::{ Terminal, TerminalConst, Widget};
use crate::terminal::{Rectangle, UpdateInfo, UpdateResult};
use crate::widgets::BoundingBox;

/// The [`Stacked`] widget lets you lay out one widget on top of another.
pub struct Stacked<TOP, BOT> {
    /// The widget that is on top.
    pub higher_widget: TOP,
    /// The widget that is on the bottom.
    pub lower_widget: BOT,
    /// The leftover update from the last [`Widget::update`] call.
    pub leftover_result: Option<UpdateResult>,
}

impl<TOP, BOT> Stacked<TOP, BOT> {
    /// Create a new [`Stacked`] widget.
    pub const fn new(top: TOP, bottom: BOT) -> Self {
        Self { higher_widget: top, lower_widget: bottom, leftover_result: None }
    }

    /// Draws the bottom widget, and returns its update result. This is better than using [`Widget::draw`]
    /// because it can return a leftover result from the last call to [`Widget::draw`].
    pub fn draw_bottom(&self, update_info: UpdateInfo, mut terminal: impl Terminal) -> crate::Result<UpdateResult>
    where
        TOP: BoundingBox,
        BOT: BoundingBox {
        let lower_view = self.lower_view(terminal.bounding_box())?;

        let lower_view = terminal.view_mut(lower_view).ok_or(Error::OutOfBoundsCoordinate {
            x: Some(lower_view.right()),
            y: Some(lower_view.bottom())
        })?;

        self.lower_widget.draw(update_info, lower_view)
    }

    /// Draws the top widget, and returns its update result. This is better than using [`Widget::draw`]
    /// because it can return a leftover result from the last call to [`Widget::draw`].
    ///
    /// # Errors
    ///
    /// This method will return an error if the top widget's bounding box is larger than the terminal's
    /// bounding box.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tuit::prelude::*;
    /// use tuit::terminal::ConstantSize;
    /// use tuit::widgets::builtins::{Stacked, Text};
    ///
    /// use tuit::terminal::UpdateInfo;
    /// 
    /// 
    /// let top_text = Text::new("Top text");
    /// let bottom_text = Text::new("Bottom text");
    ///
    /// let stacked = Stacked::new(top_text, bottom_text);
    ///
    /// let mut terminal: ConstantSize<20, 20> = ConstantSize::new();
    ///
    /// stacked.draw_top(UpdateInfo::NoInfo, &mut terminal).unwrap();
    /// stacked.draw_bottom(UpdateInfo::NoInfo, &mut terminal).unwrap();
    /// ```
    pub fn draw_top(&self, update_info: UpdateInfo, mut terminal: impl Terminal) -> crate::Result<UpdateResult>
    where
        TOP: BoundingBox,
        BOT: BoundingBox {
        let higher_view = self.higher_view(terminal.bounding_box())?;

        let higher_view = terminal.view_mut(higher_view).ok_or(Error::OutOfBoundsCoordinate {
            x: Some(higher_view.right()),
            y: Some(higher_view.bottom())
        })?;

        self.higher_widget.draw(update_info, higher_view)
    }

    /// Draws both widgets, and returns their update results. This is better than using [`Widget::draw`]
    /// because it returns draw update results from both widgets.
    pub fn draw_both(&self, update_info: UpdateInfo, mut terminal: impl Terminal) -> (crate::Result<UpdateResult>, crate::Result<UpdateResult>)
    where
        TOP: BoundingBox,
        BOT: BoundingBox {
        let res_higher = self.draw_top(update_info, &mut terminal);
        let res_lower = self.draw_bottom(update_info, &mut terminal);

        (res_higher, res_lower)
    }

    fn higher_view(&self, bounds: Rectangle) -> crate::Result<Rectangle>
    where TOP: BoundingBox {
        // We're not really moving this around, so we can just call the bounding box method.
        // Lol.
        self.higher_widget.bounding_box(bounds)
    }
    
    fn lower_view(&self, bounds: Rectangle) -> crate::Result<Rectangle>
    where
        TOP: BoundingBox,
        BOT: BoundingBox {
        // Shorter variable names are welcome. Please submit a PR if you can think of a better name.
        let higher_view_left_bottom = self.higher_view(bounds)?.left_bottom();
        let lower_view_uncorrected = self.lower_widget.bounding_box(bounds)?;
        let lower_view_corrected = lower_view_uncorrected.at(higher_view_left_bottom);

        if !bounds.contains_rect(lower_view_corrected) {
            return Err(Error::RequestRescale {
                new_width: lower_view_corrected.right(),
                new_height: lower_view_corrected.bottom(),
            })
        }

        Ok(lower_view_corrected)
    }
}

impl<TOP: BoundingBox, BOT: BoundingBox> Widget for Stacked<TOP, BOT> {
    fn update(&mut self, update_info: UpdateInfo, terminal: impl TerminalConst) -> crate::Result<UpdateResult> {
        let higher_view = self.higher_view(terminal.bounding_box())?;
        let lower_view = self.lower_view(terminal.bounding_box())?;

        let higher_view = terminal.view(higher_view).ok_or(Error::OutOfBoundsCoordinate {
            x: Some(higher_view.right()),
            y: Some(higher_view.bottom())
        })?;

        let higher_update = self.higher_widget.update(update_info, higher_view);

        let lower_view = terminal.view(lower_view).ok_or(Error::OutOfBoundsCoordinate {
            x: Some(lower_view.right()),
            y: Some(lower_view.bottom())
        })?;

        let lower_update = self.lower_widget.update(update_info, lower_view);

        let res_higher = higher_update?;
        let res_lower = lower_update?;

        self.leftover_result = Some(res_lower.min(res_higher));

        Ok(res_lower.max(res_higher))
    }

    fn draw(&self, update_info: UpdateInfo, mut terminal: impl Terminal) -> crate::Result<UpdateResult> {
        let res_higher = self.draw_top(update_info, &mut terminal)?;
        let res_lower = self.draw_bottom(update_info, &mut terminal)?;

        Ok(res_lower.max(res_higher))
    }
}

impl<TOP: BoundingBox, BOT: BoundingBox> BoundingBox for Stacked<TOP, BOT> {
    fn bounding_box(&self, terminal_rect: Rectangle) -> crate::Result<Rectangle> {
        let top_height = self.higher_widget.bounding_box(terminal_rect)?.height();
        let bottom_height = self.lower_widget.bounding_box(terminal_rect)?.height();

        let top_width = self.higher_widget.bounding_box(terminal_rect)?.width();
        let bottom_width = self.lower_widget.bounding_box(terminal_rect)?.width();

        let width = top_width.max(bottom_width);
        let height = top_height + bottom_height;

        let left_top = self.higher_widget.bounding_box(terminal_rect)?.left_top();

        let rect = Rectangle::of_size((width, height)).at(left_top);

        if !terminal_rect.contains_rect(rect) {
            return Err(Error::RequestRescale {
                new_width: rect.width().max(width),
                new_height: rect.height().max(height),
            });
        }

        Ok(rect)
    }

    fn completely_covers(&self, bounds: Rectangle) -> bool {
        let Ok(rect) = self.bounding_box(bounds) else {
            return false;
        };

        let Ok(higher_view) = self.higher_view(bounds) else {
            return false;
        };

        let Ok(lower_view) = self.lower_view(bounds) else {
            return false;
        };

        rect.dimensions() == bounds.dimensions()
            && self.lower_widget.completely_covers(lower_view)
            && self.higher_widget.completely_covers(higher_view)
    }
}