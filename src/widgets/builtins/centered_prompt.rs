use crate::Error;
use crate::prelude::*;
use crate::style::Style;
use crate::terminal::{UpdateInfo, UpdateResult};
use crate::widgets::BoundingBox;
use crate::widgets::builtins::centered_text::CenteredText;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
/// A prompt that can be configured with several buttons
/// 
/// # Panics
/// 
/// Rendering panics if any button is wider than the terminal's width.
/// Make sure to truncate your button text so that it only fits on one line.
pub struct CenteredPrompt<'a> {
    /// The body text of the prompt.
    pub centered_text: CenteredText<'a>,
    /// An array containing the text values of each button.
    pub buttons: &'a [&'a str],
    /// The Cell Style for selected buttons. Leave `None` to use the same style as the text body.
    pub selected_button_style: Style,
    /// The Cell Style for unselected buttons. Leave `None` to use the same style as the text body.
    pub unselected_button_style: Style,
    /// The button currently hovered over. `None` means no button is selected.
    hovered_button: Option<usize>,
}

impl<'a> CenteredPrompt<'a> {
    /// Create a new [`CenteredPrompt`] with the specified buttons.
    #[must_use]
    pub const fn new(text: &'a str, buttons: &'a [&'a str]) -> Self {
        Self {
            centered_text: CenteredText::new(text),
            selected_button_style: Style::new().underlined(),
            unselected_button_style: Style::new(),
            hovered_button: None,
            buttons,
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

impl Widget for CenteredPrompt<'_> {
    fn update(
        &mut self,
        _update_info: UpdateInfo,
        _terminal: impl TerminalConst,
    ) -> crate::Result<UpdateResult> {
        Err(Error::Todo)
    }

    fn draw(
        &self,
        update_info: UpdateInfo,
        mut terminal: impl Terminal,
    ) -> crate::Result<UpdateResult> {
        self.centered_text.draw(update_info, &mut terminal)?; // Draw text.

        let term_width = terminal.width();
        let bottom = self.centered_text.bounding_box_in(&terminal)?.bottom();

        let mut characters_used = self.buttons.first().unwrap_or(&"").len();
        let term_chars = terminal.cells_mut();

        let lines = self.buttons.split_inclusive(|button_text| {
            if button_text.len() > term_width {
                todo!(
                    "Failed to handle edge case properly... \
                (This occurs when a button's text length is greater than the terminal width in a \
                `CenteredPrompt` dialogue)"
                )
            }

            if characters_used + button_text.len() >= term_width {
                characters_used = button_text.len();
                return true;
            }

            characters_used += button_text.len();

            false
        });

        let mut current_button = 0;

        let mut term_chars = term_chars.skip(bottom * term_width);


        for (line_offset, buttons) in lines.enumerate() {
            let mut col_no = 0;
            // pluh? pluh 🗣
            for button in buttons {
                col_no += button.len();
            }

            let mut cursor =
                (term_width
                    .checked_sub(col_no)
                    .ok_or(Error::OutOfBoundsCoordinate {
                        x: Some(col_no),
                        y: Some(line_offset + bottom),
                    })?)
                    / 2; // find middle.

            let mut term_chars = term_chars.by_ref().skip(cursor);

            let text_style = self.centered_text.style;

            for button in buttons {
                for character in button.chars() {
                    let current_cell = term_chars.next().ok_or(Error::OutOfBoundsCoordinate {
                        x: Some(col_no),
                        y: Some(line_offset + bottom),
                    })?;

                    let base_style = text_style.inherits(current_cell.style);

                    current_cell.character = character;

                    // if button is hovered, apply hovered button style. otherwise, apply standard
                    // style.
                    if Some(current_button) == self.hovered_button {
                        current_cell.style = self.selected_button_style.inherits(base_style);
                    } else {
                        current_cell.style = self.unselected_button_style.inherits(base_style);
                    }

                    cursor += 1;
                }
                current_button += 1;
            }
            term_chars.nth(term_width - cursor - 1);
        }

        Ok(UpdateResult::NoEvent)
    }
}