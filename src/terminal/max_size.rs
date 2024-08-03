use core::array;

use crate::prelude::*;
use crate::style::Style;
use crate::terminal::{Cell, Metadata, Rescalable};

/// A zero-allocation re-scalable terminal that allocates the maximum size that it can scale to.
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct MaxSize<const MAX_WIDTH: usize, const MAX_HEIGHT: usize> {
    characters: [[Cell; MAX_WIDTH]; MAX_HEIGHT],
    default_style: Style,
    dimensions: (usize, usize),
}

impl<const MAX_WIDTH: usize, const MAX_HEIGHT: usize> Default for MaxSize<MAX_WIDTH, MAX_HEIGHT> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const MAX_WIDTH: usize, const MAX_HEIGHT: usize> MaxSize<MAX_WIDTH, MAX_HEIGHT> {
    /// Creates a new [`MaxSize`] at its maximum size.
    #[must_use]
    pub fn new() -> Self {
        Self {
            characters: array::from_fn(|_| array::from_fn(|_| Cell::default())),
            default_style: Style::default(),
            dimensions: (MAX_WIDTH, MAX_HEIGHT),
        }
    }
}

impl<const MAX_WIDTH: usize, const MAX_HEIGHT: usize> Rescalable for MaxSize<MAX_WIDTH, MAX_HEIGHT> {
    fn rescale(&mut self, (new_width, new_height): (usize, usize)) -> Result<(), (usize, usize)> {
        let bounding_box = self.bounding_box();

        if !bounding_box.contains((new_width, new_height)) {
            return Err((
                new_width.min(MAX_WIDTH),
                new_height.min(MAX_HEIGHT)
            ));
        }

        self.dimensions = (new_width, new_height);

        Ok(())
    }
}

impl<const MAX_WIDTH: usize, const MAX_HEIGHT: usize> Metadata for MaxSize<MAX_WIDTH, MAX_HEIGHT> {
    fn dimensions(&self) -> (usize, usize) {
        self.dimensions
    }

    fn default_style(&self) -> Style {
        self.default_style
    }
}

impl<const MAX_WIDTH: usize, const MAX_HEIGHT: usize> TerminalConst
for MaxSize<MAX_WIDTH, MAX_HEIGHT>
{
    fn cells(&self) -> impl Iterator<Item = &Cell> {
        let (acting_width, acting_height) = self.dimensions;

        self.characters.iter().flatten().take(acting_width * acting_height)
    }

    fn cell(&self, x: usize, y: usize) -> Option<&Cell> {
        let (acting_width, acting_height) = self.dimensions;

        if x > acting_width {
            return None
        }

        if y > acting_height {
            return None
        }

        let row = self.characters.get(y)?;

        row.get(x)
    }
}

impl<const MAX_WIDTH: usize, const MAX_HEIGHT: usize> TerminalMut
for MaxSize<MAX_WIDTH, MAX_HEIGHT>
{
    fn cells_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        let (acting_width, acting_height) = self.dimensions;

        self.characters.iter_mut().flatten().take(acting_width * acting_height)
    }

    fn cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        let (acting_width, acting_height) = self.dimensions;
        
        if x > acting_width {
            return None
        }
        
        if y > acting_height {
            return None
        }
        
        let row = self.characters.get_mut(y)?;
        
        row.get_mut(x)
    }
}
