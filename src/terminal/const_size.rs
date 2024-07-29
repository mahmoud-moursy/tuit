use crate::prelude::{TerminalConst, TerminalMut};
use crate::terminal::{Cell, Metadata, Style};

/// A zero-allocation terminal of constant size. The terminal's size is determined at compile time,
/// and can't be changed at runtime.
///
/// ```
/// use tuit::terminal;
/// use tuit::prelude::*;
/// use tuit::widgets::CenteredText;
///
/// let mut terminal: terminal::ConstantSize<20, 20> = terminal::ConstantSize::new();
///
/// let mut widget: CenteredText = CenteredText::new("Hello world!");
///
/// // Look up `Widget::drawn` in the documentation.
/// widget.drawn(&mut terminal).expect("Should not fail!");
///
/// let std_out = std::io::stdout();
///
/// terminal.display(std_out).expect("Failed to draw terminal");
/// ```
pub struct ConstantSize<const WIDTH: usize, const HEIGHT: usize> {
    // Modifying this does not lead to UB, so they are public.
    /// The characters that are within the terminal.
    pub characters: [[Cell; WIDTH]; HEIGHT],
    /// The terminal's default style.
    pub default_style: Style,
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for ConstantSize<WIDTH, HEIGHT> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> ConstantSize<WIDTH, HEIGHT> {
    /// Creates a new [`ConstantSize`].
    ///
    /// Note that since this terminal uses const generics, the terminal's size must be defined within
    /// the type, not the function!
    ///
    /// ## Example
    ///
    /// ```
    /// use tuit::terminal::ConstantSize;
    ///
    /// // The terminal uses const-generics for compile-time evaluation.
    /// let my_terminal: ConstantSize<20, 20> = ConstantSize::new(); // does not require runtime arguments, but it needs *generic* arguments.
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            characters: [[Cell::new(' '); WIDTH]; HEIGHT],
            default_style: Style::new(),
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Metadata for ConstantSize<WIDTH, HEIGHT> {
    fn dimensions(&self) -> (usize, usize) {
        (WIDTH, HEIGHT)
    }

    fn default_style(&self) -> Style {
        self.default_style
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> TerminalConst for ConstantSize<WIDTH, HEIGHT> {
    fn characters_slice(&self) -> &[Cell] {
        self.characters.as_flattened()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> TerminalMut for ConstantSize<WIDTH, HEIGHT> {
    fn characters_slice_mut(&mut self) -> &mut [Cell] {
        self.characters.as_flattened_mut()
    }
}
