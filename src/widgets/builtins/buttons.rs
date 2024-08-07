use crate::Error;
use crate::style::{Ansi4, Colour, Style};
use crate::terminal::{Rectangle, Terminal, TerminalConst, UpdateInfo, UpdateResult};
use crate::widgets::{BoundingBox, Widget};

/// A widget that displays a list of buttons, left-to-right.
#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug, Default)]
pub struct Buttons<'a> {
    /// The buttons to display.
    pub buttons: &'a [&'a str],
    /// The style to use for the selected button.
    pub selected_button_style: Style,
    /// The style to use for the unselected buttons.
    pub unselected_button_style: Style,
    /// The index of the currently hovered button.
    pub hovered_button: Option<usize>,
}

impl<'a> Buttons<'a> {
    /// Create a new [`Buttons`] widget.
    #[must_use]
    pub const fn new(buttons: &'a [&'a str]) -> Self {
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
        self.select(self.buttons.len() - 1)
    }

    /// Selects the rightmost available button, or `None` if there are no buttons.
    ///
    /// This is an alias for [`CenteredPrompt::select_rightmost`]. Right is not last in all languages.
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
    /// This is an alias for [`CenteredPrompt::select_leftmost`]. Left is not first in all languages.
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

impl Widget for Buttons<'_> {
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
        let mut terminal_cells = terminal.cells_mut().enumerate();

        for (button_idx, button) in self.buttons.iter().enumerate() {
            let selected = Some(button_idx) == self.hovered_button;
            let base_style = if selected {
                self.selected_button_style
            } else {
                self.unselected_button_style
            };

            let max_len = button.len().min(term_bounding_box.width());

            let mut button_chars = button[..max_len].chars().enumerate().peekable();

            while let Some((chr_dep, _character)) = button_chars.peek() {
                let (idx, current_cell) = terminal_cells.next().ok_or(Error::OutOfBoundsCoordinate {
                    x: None,
                    y: None,
                })?;


                let (x, _y) = term_bounding_box.index_into(idx).ok_or(Error::OutOfBoundsCharacter(idx))?;

                if button.len() - chr_dep + x >= term_bounding_box.width() {
                    if button.len() < term_bounding_box.width() {
                        continue;
                    }
                }

                let (_dep, current_character) = button_chars.next().unwrap();

                current_cell.character = current_character;
                current_cell.style = base_style.inherits(current_cell.style);
            }
        }

        Ok(UpdateResult::NoEvent)
    }
}

impl BoundingBox for Buttons<'_> {
    fn bounding_box(&self, rect: Rectangle) -> crate::Result<Rectangle> {
        let term_bounding_box = rect;

        let (mut x, mut y) = (0, 0);
        let mut idx = 0;

        // FIXME: Optimize this so that it doesn't have to do a big ugly for loop.
        for button in self.buttons {
            let max_len = button.len().min(term_bounding_box.width());

            let mut button_chars = button[..max_len].chars().enumerate().peekable();

            while let Some((dep, _character)) = button_chars.peek() {
                let dep = *dep;
                button_chars.next().expect("This should always be Some");
                idx += 1;
                let (new_x, new_y) = term_bounding_box.index_into(idx).ok_or(Error::OutOfBoundsCharacter(idx))?;

                x = x.max(new_x);
                y = y.max(new_y);

                if button.len() + x - dep >= term_bounding_box.width() {
                    if button.len() < term_bounding_box.width() {
                        continue;
                    }
                }
            }
        }

        x += 1;

        Ok(Rectangle::of_size((x, y)))
    }

    fn completely_covers(&self, rectangle: Rectangle) -> bool {
        let term_bounding_box = rectangle;
        let mut idx = 0;

        // FIXME: Optimize this so that it doesn't have to do a big ugly for loop.
        for button in self.buttons {
            let max_len = button.len().min(term_bounding_box.width());

            let mut button_chars = button[..max_len].chars().enumerate().peekable();

            while let Some((dep, _character)) = button_chars.peek() {
                let dep = *dep;
                button_chars.next().expect("This should always be Some");
                idx += 1;
                let Some((x, _y)) = term_bounding_box.index_into(idx) else {
                    return true
                };

                if button.len() + x - dep >= term_bounding_box.width() {
                    if button.len() < term_bounding_box.width() {
                        return false;
                    }
                }
            }
        }

        true
    }
}