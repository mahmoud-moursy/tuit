use crate::prelude::*;
use crate::style::Style;
use crate::terminal::Cell;

/// A zero-allocation terminal of constant size. The terminal's size is determined at compile time,
/// and can't be changed at runtime.
///
/// ```
/// use tuit::terminal;
/// use tuit::prelude::*;
/// use tuit::widgets::builtins::CenteredText;
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
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct ConstantSize<const WIDTH: usize, const HEIGHT: usize> {
    // Modifying these values does not lead to UB or crashing, so they are public.
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
    fn cells(&self) -> impl Iterator<Item = &Cell> {
        self.characters.iter().flatten()
    }

    fn cell(&self, x: usize, y: usize) -> Option<&Cell> {
        let row = self.characters.get(y)?;

        row.get(x)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> TerminalMut for ConstantSize<WIDTH, HEIGHT> {
    fn cells_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.characters.iter_mut().flatten()
    }

    fn cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        let row = self.characters.get_mut(y)?;

        row.get_mut(x)
    }
}
