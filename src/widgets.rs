//! # Widgets
//!
//! The widgets module contains some pre-bundled widgets ready to go with Tuit.
//!
//! Every widget included is functional and document, but this is still a heavy TODO because it is very bare-bones
//! at the moment.
use crate::Error;
use crate::prelude::*;
use crate::terminal::{Colour, MouseButton, Style, Terminal, UpdateInfo, UpdateResult, Widget};

/// Provides a direction for elements
pub enum Direction {
    /// Left
    Left,
    /// Right
    Right,
    /// Down (or bottom of terminal, in some contexts)
    Down,
    /// Up (or top of terminal, in some contexts)
    Up,
}

/// A widget that will clear the entire terminal and replace it with a blank cell containing
/// the specified colour.
pub struct Sweeper {
    /// The colour to use for the blank cells that clear the terminal.
    pub colour: Colour,
}

impl Sweeper {
    /// Creates a new [`Sweeper`] with the specified `colour`
    #[must_use]
    pub const fn new(colour: Colour) -> Self {
        Self { colour }
    }
}

impl Widget for Sweeper {
    fn update(
        &mut self,
        _update_info: UpdateInfo,
        _terminal: impl TerminalConst,
    ) -> crate::Result<UpdateResult> {
        Ok(UpdateResult::NoEvent)
    }

    fn draw(
        &self,
        _update_info: UpdateInfo,
        mut terminal: impl Terminal,
    ) -> crate::Result<UpdateResult> {
        for character in terminal.characters_slice_mut() {
            character.style.bg_colour = Some(self.colour);
            character.character = ' ';
        }

        Ok(UpdateResult::NoEvent)
    }
}

/// A prompt that is centered
///
/// ```
/// use tuit::terminal::ConstantSize;
/// use tuit::widgets::CenteredText;
/// use tuit::terminal::Widget;
///
/// let small_terminal: ConstantSize<1, 1> = ConstantSize::new();
/// let my_prompt = CenteredText::new("Hello world!");
///
/// my_prompt.drawn(small_terminal).expect_err("If the terminal is too small, then an OutOfBoundsCoordinate error is returned.");
/// ```
#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct CenteredText<'a> {
    /// The text to be displayed
    pub prompt_text: &'a str,
    /// The styling behind the prompt.
    pub style: Style,
}

impl<'a> CenteredText<'a> {
    /// Initializes a [`CenteredText`] with the given text.
    ///
    /// ```
    /// use tuit::terminal::ConstantSize;
    /// use tuit::widgets::CenteredText;
    /// use tuit::prelude::*;
    ///
    /// let mut my_terminal = ConstantSize::<20, 20>::new();
    /// let my_text = CenteredText::new("Hello!");
    ///
    /// my_text
    ///     .drawn(&mut my_terminal)
    ///     .expect("This only fails if the prompt text has more characters than the terminal can contain.");
    /// ```
    #[must_use]
    pub const fn new(text: &'a str) -> Self {
        Self {
            prompt_text: text,
            style: Style::new(),
        }
    }

    /// Initializes a [`CenteredText`] with the given text.
    ///
    /// ```
    /// use tuit::terminal::{Ansi4, ConstantSize, Style};
    /// use tuit::widgets::CenteredText;
    /// use tuit::prelude::*;
    ///
    /// let mut my_terminal = ConstantSize::<20, 20>::new();
    /// let my_style = Style::new().bg_ansi4(Ansi4::Blue);
    /// let my_text = CenteredText::new("Hello!").style(my_style);
    ///
    /// my_text
    ///     .drawn(&mut my_terminal)
    ///     .expect("This only fails if the prompt text has more characters than the terminal can contain.");
    /// ```
    #[must_use]
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;

        self
    }

    /// Calculates the bounding box of the centered prompt. This method is available so that other widgets can be
    /// composed using the [`CenteredText`] widget, but it doesn't need to be used by the end-user for the TUI.
    ///
    /// [`CenteredText::bounding_box`] returns a `((usize, usize), (usize, usize))` which corresponds to x-y coordinates `((left, top), (right, bottom))`
    ///
    /// Width can be determined using `right - left`, and height can be determined using `bottom - top`. This is because
    /// the y-axis is flipped in Tuit, so `bottom` is actually the larger value, but on the x-axis, `right` is the larger
    /// value.
    #[must_use]
    pub fn bounding_box(&self, terminal: &impl TerminalConst) -> ((usize, usize), (usize, usize)) {
        let (terminal_width, terminal_height) = terminal.dimensions();

        let text_len = self.prompt_text.len();
        // Calculate the width/height of the prompt, capping it to the terminal's width.
        //    // div_ceil because if the terminal width is 12, and the text length is 13,
        //    // we want the height to be 2 because it takes 2 lines.
        let height = text_len.div_ceil(terminal_width).min(terminal_height);
        let width = text_len.min(terminal_width);

        let horizontal_center = terminal_width / 2;
        let vertical_center = terminal_height / 2;

        let left = horizontal_center - (width / 2);
        let right = left + width;

        let top = vertical_center - (height / 2);
        let bottom = top + height;

        ((left, top), (right, bottom))
    }
}

impl<'a> Widget for CenteredText<'a> {
    fn update(
        &mut self,
        update_info: UpdateInfo,
        terminal: impl TerminalConst,
    ) -> crate::Result<UpdateResult> {
        match update_info {
            UpdateInfo::CellClicked(x, y, MouseButton::LeftClick) => {
                let ((left, top), (right, bottom)) = self.bounding_box(&terminal);

                #[allow(clippy::collapsible_if)]
                // Check if click was within bounds.
                if x < left && right > x {
                    if y > top && bottom < y {
                        return Ok(UpdateResult::LifecycleEnd);
                    }
                }

                Ok(UpdateResult::NoEvent)
            }
            _ => Ok(UpdateResult::NoRedraw),
        }
    }

    fn draw(
        &self,
        _update_info: UpdateInfo,
        mut terminal: impl Terminal,
    ) -> crate::Result<UpdateResult> {
        let ((left, top), (right, _bottom)) = self.bounding_box(&terminal);
        let width = right - left;

        for (i, character) in self.prompt_text.chars().enumerate() {
            let x = (i % width) + left;
            let y = (i / width) + top;

            if let Some(cell) = terminal.character_mut(x, y) {
                cell.character = character;
                cell.style = self.style.inherits(cell.style);
            } else {
                return Err(Error::OutOfBoundsCoordinate {
                    x: Some(x),
                    y: Some(y),
                });
            }
        }

        Ok(UpdateResult::NoEvent)
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
/// A prompt that can be configured with several buttons
pub struct CenteredPrompt<'a> {
    /// The body text of the prompt.
    pub centered_text: CenteredText<'a>,
    /// An array containing the text values of each button.
    pub buttons: &'a [&'a str],
    /// The Cell Style for selected buttons. Leave `None` to use the same style as the text body.
    pub selected_button_style: Style,
    /// the Cell Style for unselected buttons. Leave `None` to use the same style as the text body.
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

        let (term_width, _term_height) = terminal.dimensions();
        let ((_left, _top), (_right, bottom)) = self.centered_text.bounding_box(&terminal);

        let mut characters_used = self.buttons.first().unwrap_or(&"").len();
        let term_chars = terminal.characters_slice_mut();

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

        for (line_offset, buttons) in lines.enumerate() {
            let start = (bottom + line_offset) * term_width;

            let mut col_no = 0;
            // pluh? pluh 🗣
            for button in buttons {
                col_no += button.len();
            }

            let mut left_offset =
                (term_width
                    .checked_sub(col_no)
                    .ok_or(Error::OutOfBoundsCoordinate {
                        x: Some(col_no),
                        y: Some(line_offset + bottom),
                    })?)
                    / 2; // find middle.

            let text_style = self.centered_text.style;

            for button in buttons {
                for character in button.chars() {
                    let current_cell = &mut term_chars[start + left_offset];

                    let base_style = text_style.inherits(current_cell.style);

                    current_cell.character = character;

                    if Some(current_button) == self.hovered_button {
                        current_cell.style = self.selected_button_style.inherits(base_style);
                    } else {
                        current_cell.style = self.unselected_button_style.inherits(base_style);
                    }

                    left_offset += 1;
                }
                current_button += 1;
            }
        }

        Ok(UpdateResult::NoEvent)
    }
}

/// This widget just marks the given [`Direction`] of the screen with x or y-coords.
///
/// For example, using this widget with [`Direction::Down`] will mark the bottom of the screen
/// with the x-coordinate of each cell.
pub struct Ruler(u32, pub Direction);

impl Default for Ruler {
    fn default() -> Self {
        Self(16, Direction::Down)
    }
}
impl Ruler {
    /// Initializes a Ruler with the given radix.
    ///
    /// Returns [`None`] if supplied radix is invalid (radix must be in range 2..=36)
    #[must_use]
    pub const fn new(radix: u32, direction: Direction) -> Option<Self> {
        if radix <= 36 && radix >= 2 {
            Some(Self(radix, direction))
        } else {
            None
        }
    }

    fn horizontal_draw(&self, mut terminal: impl Terminal) {
        let (width, height) = terminal.dimensions();
        let characters = terminal.characters_slice_mut();

        let bar = match self.1 {
            Direction::Up => &mut characters[..width],
            Direction::Down => &mut characters[(height - 1) * width..],
            _ => unreachable!(),
        };

        #[allow(clippy::cast_possible_truncation)]
        for (x, character) in bar.iter_mut().enumerate() {
            // Truncation here is impossible, unless you are on an architecture below 32-bits.
            character.character = char::from_digit(x as u32 % self.0, self.0)
                .expect("Should never fail. Tried to convert an invalid digit into a character!");
        }
    }

    fn vertical_draw(&self, mut terminal: impl Terminal) {
        let (width, height) = terminal.dimensions();
        let characters = terminal.characters_slice_mut();

        let x_offset = match self.1 {
            Direction::Left => 0,
            Direction::Right => width - 1,
            _ => unreachable!(),
        };

        #[allow(clippy::cast_possible_truncation)]
        for y in 0..height {
            characters[(y * width) + x_offset].character =
                char::from_digit(y as u32 % self.0, self.0).expect(
                    "Should never fail. Tried to convert an invalid digit into a character!",
                );
        }
    }
}

impl Widget for Ruler {
    fn update(
        &mut self,
        _update_info: UpdateInfo,
        _terminal: impl TerminalConst,
    ) -> crate::Result<UpdateResult> {
        Ok(UpdateResult::NoEvent)
    }

    fn draw(
        &self,
        _update_info: UpdateInfo,
        terminal: impl Terminal,
    ) -> crate::Result<UpdateResult> {
        match self.1 {
            Direction::Up | Direction::Down => self.horizontal_draw(terminal),
            Direction::Left | Direction::Right => self.vertical_draw(terminal),
        }

        Ok(UpdateResult::NoEvent)
    }
}

// #[cfg(feature = "alloc")]
// pub mod alloc {
//     //! This module is for widgets that rely upon allocation to function.
//     //! The module is only included if the crate's `alloc` feature is enabled!
//
//     extern crate alloc;
// }

#[cfg(test)]
#[doc(hidden)]
/// Proud to be a great programmer who tests his code. :)
mod test {}
