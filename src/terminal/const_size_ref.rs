use crate::prelude::*;
use crate::style::Style;
use crate::terminal::Cell;
#[allow(unused_imports)]
// is used for rustdoc.
use crate::terminal::ConstantSize;

/// An implementation of the [`ConstantSize`] that can be created from mutable references to arrays.
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct ConstantSizeRef<const WIDTH: usize, const HEIGHT: usize, T> {
    // Modifying this does not lead to UB, so they are public.
    /// The characters that are within the terminal.
    pub characters: T,
    /// The terminal's default style.
    pub default_style: Style,
}

// That certainly is a mouthful...
impl<const WIDTH: usize, const HEIGHT: usize, T> ConstantSizeRef<WIDTH, HEIGHT, T> {
    /// Creates a new [`ConstantSizeRef`] with the specified terminal
    #[must_use]
    pub const fn new(terminal: T) -> Self {
        Self {
            characters: terminal,
            default_style: Style::new(),
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, T> Metadata for ConstantSizeRef<WIDTH, HEIGHT, T>
where
    T: AsRef<[[Cell; WIDTH]; HEIGHT]>,
{
    fn dimensions(&self) -> (usize, usize) {
        (WIDTH, HEIGHT)
    }

    fn default_style(&self) -> Style {
        self.default_style
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, T> TerminalConst for ConstantSizeRef<WIDTH, HEIGHT, T>
where
    T: AsRef<[[Cell; WIDTH]; HEIGHT]>,
{
    fn characters_slice(&self) -> &[Cell] {
        self.characters.as_ref().as_flattened()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, T> TerminalMut for ConstantSizeRef<WIDTH, HEIGHT, T>
where
    T: AsMut<[[Cell; WIDTH]; HEIGHT]> + AsRef<[[Cell; WIDTH]; HEIGHT]>,
{
    fn characters_slice_mut(&mut self) -> &mut [Cell] {
        self.characters.as_mut().as_flattened_mut()
    }
}
