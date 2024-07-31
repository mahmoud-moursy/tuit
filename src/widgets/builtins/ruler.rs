use crate::prelude::*;
use crate::terminal::{UpdateInfo, UpdateResult};
use crate::widgets::Direction;

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