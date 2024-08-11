//! A probably inefficient implementation of a dynamically allocating, rescalable terminal.
//!
//! It is bundled in `extra` because it pulls in the `ndarray` dependency.

use ndarray::{Array, Array2, Axis, Dim, Shape};

use crate::prelude::{Metadata, TerminalMut};
use crate::style::Style;
use crate::terminal::{Cell, Rescalable, TerminalConst};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
/// A dynamically-allocating, fully rescalable terminal.
pub struct Rescale {
    cells: Array2<Cell>,
    default_style: Style
}

impl Rescale {
    /// Create a new [`Rescale`] terminal based on the given width and height.
    #[must_use] pub fn new((width, height): (usize, usize)) -> Self {
        Self {
            cells: Array2::default((height, width)),
            default_style: Style::new()
        }
    }
}

impl Metadata for Rescale {
    fn dimensions(&self) -> (usize, usize) {
        let [height, width] = self.cells.shape() else {
            unreachable!()
        };

        (*width, *height)
    }

    fn default_style(&self) -> Style {
        self.default_style
    }
}

impl TerminalConst for Rescale {
    fn cells(&self) -> impl Iterator<Item=&Cell> {
        self.cells.iter()
    }

    fn cell(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get((x, y))
    }
}

impl TerminalMut for Rescale {
    fn cells_mut(&mut self) -> impl Iterator<Item=&mut Cell> {
        self.cells.iter_mut()
    }

    fn cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.cells.get_mut((x, y))
    }
}

impl Rescalable for Rescale {
    fn rescale(&mut self, (new_width, new_height): (usize, usize)) -> Result<(), (usize, usize)> {
        while self.width() < new_width {
            self.cells.push_column(
                Array::from_shape_fn(
                    Shape::from(Dim([self.height()])),
                    |_x| {
                        Cell::default()
                    }
                ).view()
            ).expect("Shape should always be correct.");
        }

        while self.height() < new_height {
            self.cells.push_row(
                Array::from_shape_fn(
                    Shape::from(Dim([self.width()])),
                    |_x| {
                        Cell::default()
                    }
                ).view()
            ).expect("Shape should always be correct.");
        }

        while self.height() > new_height {
            self.cells.remove_index(Axis(0), self.height()-1);
        }
        while self.width() > new_width {
            self.cells.remove_index(Axis(1), self.width()-1);
        }

        Ok(())
    }
}