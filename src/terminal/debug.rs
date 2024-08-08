use crate::draw::Renderer;
use crate::prelude::Metadata;
use crate::style::Style;
use crate::terminal::{Cell, Terminal, TerminalConst, TerminalMut};

/// Print every step of the terminal's draw process out.
#[derive(Debug, Clone, Copy)]
pub struct Debug<T, D> {
    /// The terminal to debug.
    pub terminal: T,
    /// The renderer to use.
    pub display: D,
}

impl<T: Terminal, D: Renderer> Debug<T, D> {
    /// Create a new [`Debug`] wrapper.
    #[must_use]
    pub const fn new(terminal: T, display: D) -> Debug<T, D> {
        Debug { terminal, display }
    }

    /// Get the inner terminal.
    #[must_use]
    pub const fn terminal(&self) -> &T {
        &self.terminal
    }
}

#[cfg(feature = "std")]
use crate::std::stdout_render::StdoutRender;

#[cfg(feature = "std")]
impl<T: Terminal> Debug<T, StdoutRender> {
    /// Make a StdoutTerminal wrapper.
    pub fn stdout(terminal: T) -> Self {
        Self::new(terminal, StdoutRender::default())
    }
}

impl<T: Metadata, D> Metadata for Debug<T, D> {
    fn dimensions(&self) -> (usize, usize) {
        self.terminal.dimensions()
    }

    fn default_style(&self) -> Style {
        self.terminal.default_style()
    }
}

impl<T: Terminal, D: Renderer> TerminalMut for Debug<T, D> {
    fn cells_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        use crate::style::Colour;
        use crate::style::Ansi4;

        self
            .terminal
            .cells_mut()
            .map(|cell| {
                cell.style = cell.style.bg(Colour::Ansi16(Ansi4::Red));
                cell
            })
    }

    fn cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.display.render(&self.terminal).ok();

        self.terminal.cell_mut(x, y)
    }
}

impl<T: Terminal, D: Renderer> TerminalConst for Debug<T, D> {
    fn cells(&self) -> impl Iterator<Item = &Cell> {
        self.terminal.cells()
    }

    fn cell(&self, x: usize, y: usize) -> Option<&Cell> {
        self.terminal.cell(x, y)
    }
}