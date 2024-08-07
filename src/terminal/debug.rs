use crate::draw::Renderer;
use crate::prelude::Metadata;
use crate::style::Style;
use crate::terminal::{Cell, Terminal, TerminalMut};

/// Print every step of the terminal's draw process out.
#[derive(Debug, Clone, Copy)]
pub struct Debug<T: Terminal, D: Renderer> {
    pub terminal: T,
    pub display: D,
}

impl<T: Terminal, D: Renderer> Debug<T, D> {
    /// Create a new [`Debug`] wrapper.
    #[must_use]
    pub const fn new(terminal: T, display: D) -> Self {
        Self { terminal, display }
    }
}

impl<T: Terminal, D: Renderer> Metadata for Debug<T, D> {
    fn dimensions(&self) -> (usize, usize) {
        self.terminal.dimensions()
    }

    fn default_style(&self) -> Style {
        self.terminal.default_style()
    }
}

impl<T: Terminal, D: Renderer> TerminalMut for Debug<T, D> {
    fn cells_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        // TODO: It does not render the terminal on every access to the cells.

        self
            .terminal
            .cells_mut()
    }

    fn cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.display.render(&self.terminal).ok();

        self.terminal.cell_mut(x, y)
    }
}

// TODO: Implement this.
#[allow(dead_code)]
struct DebugIterator {

}