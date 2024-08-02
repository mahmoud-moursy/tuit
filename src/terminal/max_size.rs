use core::array;

use crate::prelude::*;
use crate::style::Style;
use crate::terminal::{Cell, Metadata};

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

    /// Rescales the terminal if the new specified width and height are below the maximum limits.
    /// Otherwise, it will return an Err, containing the values put into it.
    ///
    /// It is wise to redraw the terminal after doing this, since [`MaxSize`] will completely reorder all characters
    /// after doing this.
    ///
    /// ```
    /// use tuit::terminal::MaxSize;
    /// use tuit::prelude::*;
    ///
    /// let mut my_max_terminal: MaxSize<20, 20> = MaxSize::new();
    ///
    /// my_max_terminal.rescale(10, 10).expect("This won't fail because the size is below the limit.");
    ///
    /// let (overflowing_width, height) = my_max_terminal.rescale(21, 10).expect_err("This must always be an error!");
    ///
    /// assert_eq!(overflowing_width, 21);
    /// assert_eq!(height, 10);
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an [`Err`] when the specified new width and height are larger than the maximum allocated amount.
    pub fn rescale(&mut self, new_width: usize, new_height: usize) -> Result<(), (usize, usize)> {
        if new_width > MAX_WIDTH {
            return Err((new_width, new_height));
        }

        if new_height > MAX_HEIGHT {
            return Err((new_width, new_height));
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
