//! # Widgets
//!
//! The widgets module contains some pre-bundled widgets ready to go with Tuit.
//!
//! Every widget included is functional and document, but this is still a heavy TODO because it is very bare-bones
//! at the moment.
use crate::prelude::*;
use crate::terminal::{Colour, MouseButton, Style, UpdateInfo, UpdateResult, Widget};
use crate::Error;

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
        _terminal: impl Terminal,
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
pub struct CenteredText<'a> {
    /// The text to be displayed
    pub prompt_text: &'a str,
    /// The styling behind the prompt.
    pub style: Style
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
            style: Style::new()
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
    #[must_use] pub const fn style(mut self, style: Style) -> Self {
        self.style = style;

        self
    }

    /// Calculates the bounding box of the centered prompt. This method is available so that other widgets can be
    /// composed using the [`CenteredText`] widget, but it doesn't need to be used by the end-user for the TUI.
    ///
    /// [`CenteredText::calculate_dimensions`] returns a `((usize, usize), (usize, usize))` which corresponds to x-y coordinates `((left, top), (right, bottom))`
    ///
    /// Width can be determined using `right - left`, and height can be determined using `bottom - top`. This is because
    /// the y-axis is flipped in Tuit, so `bottom` is actually the larger value, but on the x-axis, `right` is the larger
    /// value.
    #[must_use] pub fn calculate_dimensions(&self, terminal: &impl Terminal) -> ((usize, usize), (usize, usize)) {
        let (terminal_width, terminal_height) = terminal.dimensions();

        let text_len = self.prompt_text.len();
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
        terminal: impl Terminal,
    ) -> crate::Result<UpdateResult> {
        match update_info {
            UpdateInfo::CellClicked(x, y, MouseButton::LeftClick) => {
                let ((left, top), (right, bottom)) = self.calculate_dimensions(&terminal);

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
        let ((left, top), (right, _bottom)) = self.calculate_dimensions(&terminal);
        let width = right - left;

        for (i, character) in self.prompt_text.chars().enumerate() {
            let x = (i % width) + left;
            let y = (i / width) + top;

            if let Some(cell) = terminal.character_mut(x, y) {
                cell.character = character;
                cell.style = cell.style.inherits(self.style);
            } else {
                return Err(Error::OutOfBoundsCoordinate(x, y));
            }
        }

        Ok(UpdateResult::NoEvent)
    }
}

/// A prompt that can be configured with several buttons
pub struct CenteredPrompt<'a> {
    // The child centered_text, containing the prompt text.
    centered_text: CenteredText<'a>,
    /// An array containing the text values of each button.
    pub buttons: &'a [&'a str],
    // The button currently hovered over
    button_hovered: usize,
}

impl<'a> CenteredPrompt<'a> {
    #[must_use] pub const fn new(text: &'a str, buttons: &'a [&'a str]) -> Self {
        Self {
            centered_text: CenteredText::new(text),
            buttons,
            button_hovered: 0,
        }
    }
}

/// This widget just marks the given [`Direction`] of the screen with x or y-coords.
///
/// For example, using this widget with [`Direction::Down`] will mark the bottom of the screen
/// with the x-coordinate of each cell.
pub struct Ruler(u32, Direction);

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
            Direction::Up => {
                &mut characters[..width]
            }
            Direction::Down => {
                &mut characters[(height - 1) * width..]
            }
            _ => unreachable!(),
        };

        for (x, character) in bar.iter_mut().enumerate() {
            // Truncation here is impossible, unless you are on an architecture below 32-bits.
            character.character =
                char::from_digit(
                    u32::try_from(x % self.0 as usize)
                        .expect("usize is too small (applies to 16-bit or below architectures)"),
                    self.0
                ).expect("Should never fail.");
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

        for y in 0..height {
            // Truncation here is impossible, unless you are on an architecture below 32-bits.
            characters[(y * width) + x_offset].character =
                char::from_digit(
                    u32::try_from(y % self.0 as usize)
                        .expect("usize is too small (applies to 16-bit or below architectures)"),
                    self.0
                ).expect("Radix overflowed, or wrapping has failed.");
        }
    }
}

impl Widget for Ruler {
    fn update(
        &mut self,
        _update_info: UpdateInfo,
        _terminal: impl Terminal,
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

#[cfg(feature = "alloc")]
pub mod alloc {
    //! This module is for widgets that rely upon allocation to function.
    //! The module is only included if the crate's `alloc` feature is enabled!

    extern crate alloc;
}

#[cfg(test)]
#[doc(hidden)]
mod test {}
