use crate::Error;
use crate::style::Style;
use crate::terminal::{Rectangle, Terminal, TerminalConst, UpdateInfo, UpdateResult};
use crate::widgets::{BoundingBox, Widget};

/// A widget that displays a list of buttons, left-to-right.
#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug, Default)]
pub struct Buttons<'a, T> {
    /// The buttons to display.
    pub buttons: &'a [T],
    /// The style to use for the selected button.
    pub selected_button_style: Style,
    /// The style to use for the unselected buttons.
    pub unselected_button_style: Style,
    /// The index of the currently hovered button.
    pub hovered_button: Option<usize>,
}

impl<'a, T: AsRef<str>> Buttons<'a, T> {
    /// Create a new [`Buttons`] widget.
    #[must_use]
    pub const fn new(buttons: &'a [T]) -> Self {
        Self {
            buttons,
            selected_button_style: Style::new(),
            unselected_button_style: Style::new(),
            hovered_button: None,
        }
    }

    #[must_use]
    /// Returns the currently selected button. Will be `None` if no button is selected.
    pub const fn selected(&self) -> Option<usize> {
        self.hovered_button
    }

    /// Select a button based on its order from left-to-right.
    #[must_use]
    pub const fn select(mut self, selection: usize) -> Self {
        if selection < self.buttons.len() {
            self.hovered_button = Some(selection);
        }

        self
    }

    /// Deselects button.
    #[must_use]
    pub const fn select_none(mut self) -> Self {
        self.hovered_button = None;

        self
    }

    /// Selects the rightmost available button, or `None` if there are no buttons.
    #[must_use]
    pub const fn select_rightmost(self) -> Self {
        let Some(len) = self.buttons.len().checked_sub(1) else {
            return self.select_none();
        };

        self.select(len)
    }

    /// Selects the rightmost available button, or `None` if there are no buttons.
    ///
    /// This is an alias for [`Buttons::select_rightmost`]. Right is not last in all languages.
    #[must_use]
    pub const fn select_last(self) -> Self {
        self.select_rightmost()
    }

    /// Selects the leftmost button, or `None` if there are no buttons.
    #[must_use]
    pub const fn select_leftmost(self) -> Self {
        self.select(0)
    }

    /// Selects the leftmost button, or `None` if there are no buttons.
    ///
    /// This is an alias for [`Buttons::select_leftmost`]. Left is not first in all languages.
    #[must_use]
    pub const fn select_first(self) -> Self {
        self.select_leftmost()
    }

    /// Selects the button to the right of the cursor, or `None` if there are no buttons.
    ///
    /// If the cursor position is unset, it will select the rightmost item.
    #[must_use]
    pub const fn move_right(self) -> Self {
        let Some(mut selected) = self.hovered_button else {
            return self.select_rightmost();
        };

        selected += 1;

        self.select(selected)
    }

    /// Selects the button to the right of the cursor, or `None` if there are no buttons.
    ///
    /// If the cursor position is unset, it will select the leftmost item.
    #[must_use]
    pub const fn move_left(self) -> Self {
        let Some(selected) = self.hovered_button else {
            return self.select_leftmost();
        };

        let Some(selected) = selected.checked_sub(1) else {
            return self.select_leftmost();
        };

        self.select(selected)
    }
}

impl<T: AsRef<str>> Widget for Buttons<'_, T> {
    fn update(
        &mut self,
        _update_info: UpdateInfo,
        _terminal: impl TerminalConst,
    ) -> crate::Result<UpdateResult> {
        Err(Error::Todo)
    }

    fn draw(
        &self,
        _update_info: UpdateInfo,
        mut terminal: impl Terminal,
    ) -> crate::Result<UpdateResult> {
        let term_bounding_box = terminal.bounding_box();
        let mut terminal_cells = terminal.cells_mut().enumerate().peekable();

        for (button_idx, button) in self.buttons.iter().enumerate() {
            let selected = Some(button_idx) == self.hovered_button;
            let base_style = if selected {
                self.selected_button_style
            } else {
                self.unselected_button_style
            };

            let max_len = button.as_ref().len().min(term_bounding_box.width());

            let (next_idx, _next_cell) = terminal_cells.peek().ok_or(Error::OutOfBoundsCoordinate {
                x: None,
                y: None,
            })?;

            let (cursor_x, cursor_y) = term_bounding_box.index_into(*next_idx).ok_or(Error::OutOfBoundsCharacter(*next_idx))?;

            let button_chars = button.as_ref()[..max_len].chars().peekable();

            if button.as_ref().len() + cursor_x > term_bounding_box.width() {
                // Skips until next line, only if the button fits on one line.
                if button.as_ref().len() <= term_bounding_box.width() {
                    while let Some((idx, _cell)) = terminal_cells.peek() {
                        let (_x, y) = term_bounding_box.index_into(*idx).ok_or(Error::OutOfBoundsCharacter(*idx))?;

                        if y != cursor_y {
                            break;
                        }

                        terminal_cells.next();
                    }
                }
                // If it does not fit on one line, then we need to truncate the button.
                // We already did this above, so we don't need to do it again.
            }

            for current_character in button_chars {
                let (_idx, current_cell) = terminal_cells.next().ok_or(Error::OutOfBoundsCoordinate {
                    x: None,
                    y: None,
                })?;

                current_cell.character = current_character;
                current_cell.style = base_style.inherits(current_cell.style);
            }
        }

        Ok(UpdateResult::NoEvent)
    }
}

impl<T: AsRef<str>> BoundingBox for Buttons<'_, T> {
    fn bounding_box(&self, rect: Rectangle) -> crate::Result<Rectangle> {
        let term_bounding_box = rect;

        let (mut width, mut height) = (0, 0);
        let mut idx = 0;

        height += 1; // Account for the first line.

        // FIXME: Optimize this so that it doesn't have to do a big ugly for loop.
        // We literally just copy the code from the draw method, but change it so that it logs
        // the furthest out x and y coordinates. This is probably not the most efficient way to do
        // this, but it's the only way I can think of right now.
        for button in self.buttons {
            let max_len = button.as_ref().len().min(term_bounding_box.width());
            let next_idx = idx + 1;

            let (cursor_x, cursor_y) = rect.index_into(next_idx).ok_or(Error::OutOfBoundsCharacter(next_idx))?;

            let button_chars = button.as_ref()[..max_len].chars().enumerate().peekable();

            if button.as_ref().len() + cursor_x > rect.width() {
                // Skips until next line, only if the button fits on one line.
                if button.as_ref().len() <= rect.width() {
                    while let Some((x, y)) = rect.index_into(idx) {
                        if y != cursor_y {
                            break;
                        }

                        idx += 1;
                    }
                }
                // If it does not fit on one line, then we need to truncate the button.
                // We already did this above, so we don't need to do it again.
            }

            for _current_character in button_chars {
                idx += 1;

                let (x, y) = rect.index_into(idx).ok_or(Error::OutOfBoundsCharacter(idx))?;

                width = width.max(x+1);
                height = height.max(y);
            }

            let (x, y) = rect.index_into(idx).ok_or(Error::OutOfBoundsCharacter(idx))?;

            width = width.max(x);
            height = height.max(y);
        }

        Ok(Rectangle::of_size((width, height)))
    }

    fn completely_covers(&self, rectangle: Rectangle) -> bool {
        let term_bounding_box = rectangle;
        let mut idx = 0;

        // FIXME: Optimize this so that it doesn't have to do a big ugly for loop.
        // We literally just copy the code from the draw method, but change it so that diverges
        // early if any Cell has been skipped.
        for button in self.buttons {
            let max_len = button.as_ref().len().min(term_bounding_box.width());

            let mut button_chars = button.as_ref()[..max_len].chars().enumerate().peekable();

            while let Some((chr_dep, _character)) = button_chars.peek() {
                let chr_dep = *chr_dep;
                idx += 1;
                let Some((x, _y)) = term_bounding_box.index_into(idx) else {
                    return true
                };

                if button.as_ref().len() + x - chr_dep >= term_bounding_box.width() {
                    if button.as_ref().len() < term_bounding_box.width() {
                        return false;
                    }
                }

                button_chars.next().expect("This should always be Some");
            }
        }

        true
    }
}