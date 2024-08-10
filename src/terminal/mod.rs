//! # Terminals
//!
//! This module encompasses the main traits needed to implement a Tuit [`Terminal`]. It also provides
//! some default terminals like the [`terminal::ConstantSize`] and the resizable [`terminal::MaxSize`].
//!
//! ## Implement a Terminal.
//!
//! The [`Terminal`] trait is a trait auto-implemented when you fulfill three criteria:
//!
//! 1. Your terminal struct has the [`Metadata`] trait
//! 2. Your terminal struct has the [`TerminalConst`] trait
//! 3. Your terminal struct has the [`TerminalMut`] trait
//!
//!
//! Here's the terminal struct we will be working with.
//!
//! ```
//! pub struct BasicTerminal;
//! ```
//!
//! ### The [`Metadata`] trait
//!
//! This trait is pretty basic. It just requires you to implement two methods:
//! - [`Metadata::dimensions`]: provide the terminal's width and height.
//! - [`Metadata::default_style`]: provide the terminal's default [`Style`].
//!
//! ```
//! use tuit::prelude::*;
//! use tuit::style::Style;
//! # pub struct BasicTerminal;
//!
//! impl Metadata for BasicTerminal {
//!     fn dimensions(&self) -> (usize, usize) {
//!         (42, 13) // width: 42 cells, height: 13 cells
//!     }
//!
//!     fn default_style(&self) -> Style {
//!         Style::new() // a style with all fields set to `None`.
//!     }
//! }
//! ```
//!
//! ### The [`TerminalConst`] trait
//!
//! This trait gets a little more complex. It basically requires you to return an immutable view
//! into the terminal's character [`Cell`]s.
//!
//! ```
//! use tuit::prelude::{Metadata, TerminalConst};
//! use tuit::terminal::Cell;
//! # use tuit::style::Style;
//! # pub struct BasicTerminal;
//! # impl Metadata for BasicTerminal {
//! #    fn dimensions(&self) -> (usize, usize) {
//! #        (42, 13) // width: 42 cells, height: 13 cells
//! #    }
//! #
//! #    fn default_style(&self) -> Style {
//! #        Style::new() // a style with all fields set to `None`.
//! #    }
//! # }
//!
//! impl TerminalConst for BasicTerminal {
//!     fn cells(&self) -> impl Iterator<Item = &Cell> {
//!         [].iter()
//!     }
//! }
//! ```
//!
//! ### The [`TerminalMut`] trait
//!
//! This trait is basically like [`TerminalConst`], but it gives a mutable view instead, so you can
//! modify each [`Cell`] if you were, for example, a widget!
//!
//! ```
//! use tuit::prelude::{Metadata, TerminalConst, TerminalMut};
//! use tuit::terminal::Cell;
//! # use tuit::style::Style;
//! # pub struct BasicTerminal;
//! # impl Metadata for BasicTerminal {
//! #    fn dimensions(&self) -> (usize, usize) {
//! #        (42, 13) // width: 42 cells, height: 13 cells
//! #    }
//! #
//! #    fn default_style(&self) -> Style {
//! #        Style::new() // a style with all fields set to `None`.
//! #    }
//! # }
//!
//! impl TerminalMut for BasicTerminal {
//!     fn cells_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
//!         [].iter_mut()
//!     }
//! }
//! ```
//!
//! ### A Complete Set.
//!
//! After implementing these three traits, we now automatically get an implementation of [`Terminal`].
//! Any immutable references to our terminal automatically get [`TerminalConst`], and mutable references
//! get a complete [`Terminal`]
//!
//! ### A more complicated terminal?
//!
//! If you're looking for some more-complicated examples, perhaps you should take a look at this module's
//! source code. A terminal like the [`ConstantSize`] terminal can be a good starting point if you are
//! well-acquainted with generics.

use core::ops::RangeInclusive;

pub use const_size::ConstantSize;
pub use const_size_ref::ConstantSizeRef;
pub use interactive::*;
pub use max_size::MaxSize;
pub use view::View;
pub use view_split::ViewSplit;

use crate::prelude::*;
use crate::style::Style;
#[allow(unused_imports)] // used in docs.
use crate::terminal;
use crate::widgets::Direction;

/// Module containing all the code required for the "interactive" aspects of Tuit. This includes code
/// like structs for handling input, like [`interactive::MouseButton`] or [`interactive::KeyState`].
pub mod interactive;


/// Code for the [`ConstantSize`] terminal.
pub mod const_size;
/// Code for the [`ConstantSizeRef`] terminal.
pub mod const_size_ref;
/// Code for the [`MaxSize`] terminal.
pub mod max_size;

/// An empty [`Terminal`] that doesn't do anything.
pub mod dummy;
/// The [`View`] terminal that can provide mutable or immutable views into terminals.
pub mod view;
/// The iterator used by the [`View`] terminal.
pub mod view_iterator;
/// The [`ViewSplit`] struct, which is used to split the terminal along its axes.
pub mod view_split;
/// The [`Debug`] terminal, which prints out the terminal's state every time it is drawn or writes
/// an Ansi4::Red to the background of modified cells.
pub mod debug;

#[cfg(feature = "owo_colors")]
mod owo_colors;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Default)]
/// This struct represents a character in the terminal (as well as all the styling that it may have)
pub struct Cell {
    /// The character inside the cell
    pub character: char,
    /// The character's styling.
    pub style: Style,
}

impl Cell {
    /// Create a new cell using the default style and the specified character.
    #[must_use]
    pub const fn new(character: char) -> Self {
        Self {
            character,
            style: Style::new(),
        }
    }
}

/// Allows you to access properties like the dimensions of a terminal and its default style.
pub trait Metadata {
    /// Returns the Terminal's dimensions in the order of (width, height).
    ///
    /// ```
    /// # use tuit::terminal::{ConstantSize, Metadata, TerminalConst};
    /// # let my_terminal: ConstantSize<1, 1> = ConstantSize::new();
    ///
    /// let (width, height) = my_terminal.dimensions();
    /// ```
    fn dimensions(&self) -> (usize, usize);

    /// Returns the Terminal's default style.
    fn default_style(&self) -> Style;

    /// Returns the terminal's width
    fn width(&self) -> usize {
        let (width, _) = self.dimensions();

        width
    }

    /// Returns the terminal's height
    fn height(&self) -> usize {
        let (_, height) = self.dimensions();

        height
    }

    /// Get a [`Rectangle`] with the width and height of the terminal. The left-top is at (0,0).
    fn bounding_box(&self) -> Rectangle {
        Rectangle::of_size((self.width(), self.height()))
    }
}

/// The [`TerminalConst`] trait allows [`Widget`]s to view a terminal's characters and style properties.
///
/// At its core, it just provides information about the terminal and const-only access to its characters.
#[allow(clippy::module_name_repetitions)]
pub trait TerminalConst: Metadata {
    /// Returns an immutable reference to the terminal's characters
    fn cells(&self) -> impl Iterator<Item = &Cell>;

    /// Retrieves an immutable reference to a terminal cell
    fn cell(&self, x: usize, y: usize) -> Option<&Cell> {
        let (width, height) = self.dimensions();

        if x >= width || y >= height {
            return None;
        }

        self.cells().nth(x + (width * y))
    }

    /// Return an immutable [`View`] into the [`Terminal`] within the specified [`Rectangle`].
    fn view(&self, rect: Rectangle) -> Option<View<&Self>> {
        View::new(self, rect)
    }

    /// You can pass any value that implements [`Renderer`] to get the terminal to update.
    ///
    /// Inversely, you can call [`Renderer::render`] on any Terminal and draw the screen
    ///
    /// ```rust
    /// use std::io::stdout;
    /// use tuit::terminal::ConstantSize;
    /// use tuit::prelude::*;
    /// use tuit::std::stdout_render::StdoutRenderer;
    ///
    /// let mut my_terminal: ConstantSize<20, 20> = ConstantSize::new();
    ///
    /// #[cfg(feature = "stdout_terminal")]
    /// my_terminal.display(StdoutRenderer::default()).expect("Failed to display the terminal");
    /// ```
    ///
    /// # Errors
    ///
    /// This will fail when the [`Renderer`] implementor experiences aa problem rendering
    fn display(&self, mut display: impl Renderer) -> crate::Result<()>
    where
        Self: Sized,
    {
        display.render(self)
    }
}

#[allow(clippy::module_name_repetitions)]
/// This trait exposes all the methods required to mutate the terminal, but it doesn't expose any
/// terminal information like the default style, the dimensions, or anything else.
pub trait TerminalMut: Metadata {
    /// Returns a mutable reference to the terminal's characters
    fn cells_mut(&mut self) -> impl Iterator<Item = &mut Cell>;

    /// Retrieves a mutable reference to a terminal character
    ///
    /// ```
    /// use tuit::terminal::ConstantSize;
    /// use tuit::prelude::*;
    /// use tuit::std::stdout_render::StdoutRenderer;
    ///
    /// let mut terminal: ConstantSize<20, 20> = ConstantSize::new();
    ///
    /// let my_character_ref = terminal.cell_mut(0, 0).expect("There should always be a character here!");
    ///
    /// // Set the top-right character to 'h'.
    /// my_character_ref.character = 'h';
    ///
    /// // NOTE: You need to enable the "stdout_render" feature for StdoutTerminal
    /// terminal.display(StdoutRenderer::default()).expect("Failed to display terminal");
    /// ```
    fn cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        let (width, height) = self.dimensions();

        if x >= width || y >= height {
            return None;
        }

        if (width * y) + x == 0 {
            self.cells_mut().next()
        } else {
            self.cells_mut().nth((width * y) + x - 1)
        }
    }

    /// Returns a mutable [`View`] into the [`Terminal`] within the specified [`Rectangle`].
    fn view_mut(&mut self, rect: Rectangle) -> Option<View<&mut Self>> {
        View::new(self, rect)
    }
}

/// This is a marker trait for types that have both [`TerminalMut`] and [`TerminalConst`].
/// It is auto-implemented for any type that implements both.
pub trait Terminal: TerminalConst + TerminalMut {}

/// Implement this for your [`Terminal`] if you want to allow it to rescale.
pub trait Rescalable {
    /// A method that tries to rescale the terminal to a new size.
    ///
    /// It is wise to redraw the terminal after doing this,
    /// since [`Rescalable`] never guarantees that the terminal
    /// screen will be in a valid state after doing this. It's likely that
    ///
    /// ```
    /// use tuit::terminal::{MaxSize, Rescalable};
    /// use tuit::prelude::*;
    ///
    /// let mut my_max_terminal: MaxSize<20, 20> = MaxSize::new();
    ///
    /// my_max_terminal.rescale((10, 10)).expect("This won't fail because the size is below the limit.");
    ///
    /// let (hint_x, hint_y) = my_max_terminal.rescale((21, 10)).expect_err("This must always be an error!");
    ///
    /// assert_eq!(hint_x, 20);
    /// assert_eq!(hint_y, 10);
    /// ```
    ///
    /// # Errors
    ///
    /// Rescaling can fail for reasons like running out of allocated memory or screen space.
    /// In this case, a size *hint* for the maximum area of the terminal is returned.
    /// This means that the terminal will *always* be smaller than the size hint, however, its minimum
    /// area may be smaller.
    fn rescale(&mut self, new_size: (usize, usize)) -> Result<(), (usize, usize)>;
}

#[derive(
    Hash,
    Eq,
    PartialEq,
    Copy,
    Clone,
    Debug,
    Default
)] // Ord and PartialOrd also implemented. Check default_impls.rs
/// Provides the edge coordinates for a tuit [`Rectangle`].
pub struct Rectangle {
    /// The top-left edge of the square.
    left_top: (usize, usize),
    /// The bottom-right edge of the square.
    right_bottom: (usize, usize),
}

impl Rectangle {
    /// Create a new [`Rectangle`] using the given set of (x,y) coordinates.
    #[must_use]
    pub const fn new(first_point: (usize, usize), second_point: (usize, usize)) -> Self {
        let (first_x, first_y) = first_point;
        let (second_x, second_y) = second_point;

        let x_smaller;
        let y_smaller;
        let x_larger;
        let y_larger;

        if first_x > second_x {
            x_larger = first_x;
            x_smaller = second_x;
        } else {
            x_larger = second_x;
            x_smaller = first_x;
        }

        if first_y > second_y {
            y_larger = first_y;
            y_smaller = second_y;
        } else {
            y_larger = second_y;
            y_smaller = first_y;
        }

        Self {
            left_top: (x_smaller, y_smaller),
            right_bottom: (x_larger, y_larger),
        }
    }
    
    /// Create a [`Rectangle`] with top-left at (0,0)
    #[must_use]
    pub const fn of_size((width, height): (usize, usize)) -> Self {
        Self {
            left_top: (0, 0),
            right_bottom: (width, height),
        }
    }

    /// Get the x-coordinate of the [`Rectangle`]'s leftmost edge.
    #[must_use]
    pub const fn left(&self) -> usize {
        self.left_top.0
    }

    /// Get the y-coordinate of the [`Rectangle`]'s top edge.
    ///
    /// In Tuit's coordinates, the y-axis is flipped -- this means that the lower you go, the higher
    /// the number becomes. Therefore, [`Rectangle::top`] is always less than [`Rectangle::bottom`]
    #[must_use]
    pub const fn top(&self) -> usize {
        self.left_top.1
    }


    /// Get the x-coordinate of the [`Rectangle`]'s rightmost edge.
    #[must_use]
    pub const fn right(&self) -> usize {
        self.right_bottom.0
    }

    /// Get the y-coordinate of the [`Rectangle`]'s bottom edge.
    ///
    /// In Tuit's coordinates, the y-axis is flipped -- this means that the lower you go, the higher
    /// the number becomes. Therefore, [`Rectangle::bottom`] is always greater than [`Rectangle::top`]
    #[must_use]
    pub const fn bottom(&self) -> usize {
        self.right_bottom.1
    }

    /// Get the top-left vertex of the [`Rectangle`].
    #[must_use]
    pub const fn left_top(&self) -> (usize, usize) {
        (self.left(), self.top())
    }

    /// Get the bottom-right vertex of the [`Rectangle`].
    #[must_use]
    pub const fn right_bottom(&self) -> (usize, usize) {
        (self.right(), self.bottom())
    }

    /// Get the bottom-left vertex of the [`Rectangle`].
    #[must_use]
    pub const fn left_bottom(&self) -> (usize, usize) {
        (self.left(), self.bottom())
    }

    /// Get the top-right vertex of the [`Rectangle`].
    #[must_use]
    pub const fn right_top(&self) -> (usize, usize) {
        (self.right(), self.top())
    }

    /// Get the width of the [`Rectangle`].
    #[must_use]
    pub const fn width(&self) -> usize {
        self.right() - self.left()
    }

    /// Get the height of the [`Rectangle`].
    #[must_use]
    pub const fn height(&self) -> usize {
        self.bottom() - self.top()
    }

    /// Returns the (width, height) of the [`Rectangle`]
    #[must_use]
    pub const fn dimensions(&self) -> (usize, usize) {
        (self.width(), self.height())
    }

    /// Get the area of the [`Rectangle`].
    #[must_use]
    pub const fn area(&self) -> usize {
        self.width() * self.height()
    }

    /// Get the edge-to-edge distance between the top-left and bottom-right vertices.
    ///
    /// ### Why isn't this `const`?
    ///
    /// Apparently the [`f32::sqrt`] function is not `const` (as of writing), so this function cannot be const.
    ///
    /// #### Additional notes
    ///
    /// With large rectangle sizes, there is a risk of some precision loss because the function converts
    /// the `usize` integers used to store lengths into `f32` floats.
    #[must_use]
    pub fn edge_to_edge(&self) -> f32 {
        #[allow(clippy::cast_precision_loss)]
        ((self.width().pow(2) + self.height().pow(2)) as f32).sqrt()
    }


    /// A method to set the right edge of the [`Rectangle`] to the specified x-coordinate.
    #[must_use]
    pub const fn right_to(mut self, new_edge: usize) -> Self {
        if new_edge >= self.left() {
            self.right_bottom.0 = new_edge;
        } else {
            // because this edge is further left than self.left_top...
            // when we set the new right edge...
            self.right_bottom.0 = new_edge;
            // it becomes further to the left than self.left_top...
            // so we swap them around so that the order is correct.
            (self.right_bottom.0, self.left_top.0) = (self.left_top.0, self.right_bottom.0);
        }

        self
    }

    /// A method to set the left edge of the [`Rectangle`] to the specified x-coordinate.
    #[must_use]
    pub const fn left_to(mut self, new_edge: usize) -> Self {
        if new_edge <= self.right() {
            self.left_top.0 = new_edge;
        } else {
            // when we set the new left edge...
            self.left_top.0 = new_edge;
            // it becomes further to the right than self.right_bottom...
            // so we swap them around so that the order is correct.
            (self.right_bottom.0, self.left_top.0) = (self.left_top.0, self.right_bottom.0);
        }

        self
    }

    /// A method to set the bottom edge of the [`Rectangle`] to the specified y-coordinate.
    #[must_use]
    pub const fn bottom_to(mut self, new_edge: usize) -> Self {
        if new_edge >= self.top() {
            self.right_bottom.1 = new_edge;
        } else {
            // when we set the bottom to the new value...
            self.right_bottom.1 = new_edge;
            // it becomes higher than self.left_top...
            // so we swap them around to make sure that the order is still correct.
            (self.left_top.1, self.right_bottom.1) = (self.right_bottom.1, self.left_top.1);
        }

        self
    }

    /// A method to set the top edge of the [`Rectangle`] to the specified y-coordinate.
    #[must_use]
    pub const fn top_to(mut self, new_edge: usize) -> Self {
        if new_edge <= self.bottom() {
            self.left_top.1 = new_edge;
        } else {
            // when we set the top to the new value...
            self.left_top.1 = new_edge;
            // it becomes lower than self.right_bottom...
            // so we swap them around to make sure that the order is still correct.
            (self.left_top.1, self.right_bottom.1) = (self.right_bottom.1, self.left_top.1);
        }

        self
    }

    /// Similar to [`Rectangle::top_to`] and its siblings, but you can select which edge to move
    /// based on the specified [`Direction`]
    #[must_use]
    pub const fn edge_to(self, new_edge: usize, edge: Direction) -> Self {
        match edge {
            Direction::Right => self.right_to(new_edge),
            Direction::Left => self.left_to(new_edge),
            Direction::Up => self.top_to(new_edge),
            Direction::Down => self.bottom_to(new_edge),
        }
    }

    /// Check if the given (x,y) coordinate is within the [`Rectangle`].
    ///
    /// ```
    /// use tuit::terminal::Rectangle;
    ///
    /// let rectangle = Rectangle::of_size((20, 20));
    /// let coordinate = (5, 5);
    ///
    /// assert!(rectangle.contains(coordinate))
    /// ```
    #[must_use]
    pub const fn contains(&self, point: (usize, usize)) -> bool {
        let (x, y) = point;
        let leftmost = self.left();
        let rightmost = self.right();

        let topmost = self.top();
        let bottommost = self.bottom();

        let x_in_bounds = x >= leftmost && x <= rightmost;
        let y_in_bounds = y >= topmost && y <= bottommost;

        x_in_bounds && y_in_bounds
    }

    /// Check if the given [`Rectangle`] is within the bounds of this [`Rectangle`].
    ///
    /// ```
    /// # use std::ops::Not;
    /// use tuit::terminal::Rectangle;
    ///
    /// let rectangle = Rectangle::of_size((20, 20));
    /// let other_rectangle = Rectangle::new((1,2), (21, 21));
    ///
    /// assert!(rectangle.contains_rect(other_rectangle).not())
    /// ```
    #[must_use]
    pub const fn contains_rect(&self, rect: Self) -> bool {
        let (left_top, right_bottom) = (rect.left_top(), rect.right_bottom());

        self.contains(left_top) && self.contains(right_bottom)
    }

    /// Moves the [`Rectangle`] to the specified position, centered around the top-left vertex.
    #[must_use]
    pub const fn at(mut self, new_left_top: (usize, usize)) -> Self {
        let width = self.width();
        let height = self.height();

        self.left_top = new_left_top;
        self.right_bottom = (self.left_top.0 + width, self.left_top.1 + height);

        self
    }

    /// Get the center of the rectangle on the x-axis.
    #[must_use]
    pub const fn center_x(&self) -> usize {
        let (left, right) = (self.left(), self.right());

        // Average of left/right
        (left + right) / 2
    }

    /// Get the center of the rectangle on the y-axis.
    #[must_use]
    pub const fn center_y(&self) -> usize {
        let (top, bottom) = (self.top(), self.bottom());

        // Average of top/bottom
        (top + bottom) / 2
    }

    /// Get the center of the rectangle.
    #[must_use]
    pub const fn center(&self) -> (usize, usize) {
        (self.center_x(), self.center_y())
    }



    /// Get the range of X values that the [`Rectangle`] spans over.
    ///
    /// ```
    /// use tuit::terminal::Rectangle;
    ///
    /// let rect = Rectangle::of_size((20, 20)).at((40, 40));
    /// let columns = rect
    ///         .range_x()
    ///         .map(|x| Rectangle::of_size((1, 20)).at((x, 40)));
    /// 
    /// let cool_x_coordinate = 57;
    ///
    /// let in_range = rect.range_x().contains(&cool_x_coordinate);
    ///
    /// assert!(in_range)
    /// ```
    #[must_use]
    pub const fn range_x(&self) -> RangeInclusive<usize> {
        self.left()..=self.right()
    }

    /// Get the range of Y values that the [`Rectangle`] spans over.
    ///
    /// ```
    /// use tuit::terminal::Rectangle;
    ///
    /// let rect = Rectangle::of_size((20, 20)).at((40, 40));
    /// let rows = rect
    ///         .range_y()
    ///         .map(|y| Rectangle::of_size((20, 1)).at((40, y)));
    ///
    /// let cool_y_coordinate = 57;
    ///
    /// let in_range = rect.range_y().contains(&cool_y_coordinate);
    ///
    /// assert!(in_range)
    /// ```
    #[must_use]
    pub const fn range_y(&self) -> RangeInclusive<usize> {
        self.top()..=self.bottom()
    }


    /// Shift the left edge of the [`Rectangle`] inwards by the specified distance.
    /// Effectively "trimming" the edge by the number of cells.
    ///
    /// ```
    /// use tuit::terminal::Rectangle;
    ///
    /// let rect = Rectangle::of_size((5, 5)).at((40, 40));
    ///
    /// let Some(rect) = rect.trim_right(20) else {
    ///     unreachable!()
    /// };
    ///
    /// assert_eq!(rect.right(), 40, "The former `left` should have become the new right side.");
    /// assert_eq!(rect.left(), 25, "Left and right should have swapped because the right shift was greater than the width.");
    /// ```
    ///
    /// # Errors
    /// Will return `None` if the edge's new X-coordinate is less than zero.
    #[must_use]
    pub const fn trim_left(self, distance: isize) -> Option<Self> {
        let Some(shift) = self.left().checked_add_signed(distance) else {
            return None
        };

        Some(self.left_to(shift))
    }

    /// Shift the right edge of the [`Rectangle`] inwards by the specified distance.
    /// Effectively "trimming" the edge by the number of cells.
    ///
    /// ```
    /// use tuit::terminal::Rectangle;
    ///
    /// let rect = Rectangle::of_size((5, 5)).at((40, 40));
    ///
    /// let Some(rect) = rect.trim_right(20) else {
    ///     unreachable!()
    /// };
    ///
    /// assert_eq!(rect.right(), 40, "The former `left` has become the new right side.");
    /// assert_eq!(rect.left(), 25, "Left and right have swapped because the right shift was greater than the width.");
    /// ```
    ///
    /// # Errors
    /// Will return `None` if the edge's new X-coordinate is less than zero.
    #[must_use]
    pub const fn trim_right(self, distance: isize) -> Option<Self> {
        let Some(shift) = self.right().checked_add_signed(-distance) else {
            return None
        };

        Some(self.right_to(shift))
    }

    /// Shift the top edge of the [`Rectangle`] inwards by the specified distance.
    /// Effectively "trimming" the edge by the number of cells.
    ///
    /// ```
    /// use tuit::terminal::Rectangle;
    ///
    /// let rect = Rectangle::of_size((5, 5)).at((40, 40));
    ///
    /// let Some(rect) = rect.trim_right(20) else {
    ///     unreachable!()
    /// };
    ///
    /// assert_eq!(rect.right(), 40, "The former `left` has become the new right side.");
    /// assert_eq!(rect.left(), 25, "Left and right have swapped because the right shift was greater than the width.");
    /// ```
    ///
    /// # Errors
    /// Will return `None` if the edge's new Y-coordinate is less than zero.
    #[must_use]
    pub const fn trim_top(self, distance: isize) -> Option<Self> {
        let Some(shift) = self.top().checked_add_signed(distance) else {
            return None
        };

        Some(self.top_to(shift))
    }

    /// Shift the bottom edge of the [`Rectangle`] inwards by the specified distance.
    /// Effectively "trimming" the edge by the number of cells.
    ///
    /// ```
    /// use tuit::terminal::Rectangle;
    ///
    /// let rect = Rectangle::of_size((5, 5)).at((40, 40));
    ///
    /// let Some(rect) = rect.trim_right(20) else {
    ///     unreachable!()
    /// };
    ///
    /// assert_eq!(rect.right(), 40, "The former `left` has become the new right side.");
    /// assert_eq!(rect.left(), 25, "Left and right have swapped because the right shift was greater than the width.");
    /// ```
    ///
    /// # Errors
    /// Will return `None` if the edge's new Y-coordinate is less than zero.
    #[must_use]
    pub const fn trim_bottom(self, distance: isize) -> Option<Self> {
        let Some(shift) = self.bottom().checked_add_signed(-distance) else {
            return None
        };

        Some(self.bottom_to(shift))
    }

    /// Trim the dimensions of the [`Rectangle`] on both the top and bottom edges.
    ///
    /// # Errors
    /// Will return `None` if either of the edges' new coordinates is below zero.
    #[must_use]
    pub const fn trim_y(self, distance: isize) -> Option<Self> {
        let Some(this) = self.trim_top(distance) else {
            return None
        };

        let Some(this) = this.trim_bottom(distance) else {
            return None
        };

        Some(this)
    }

    /// Trim the dimensions of the [`Rectangle`] on both the right and left edges.
    ///
    /// # Errors
    /// Will return `None` if either of the edges' new coordinates is below zero.
    #[must_use]
    pub const fn trim_x(self, distance: isize) -> Option<Self> {
        let Some(this) = self.trim_left(distance) else {
            return None
        };

        let Some(this) = this.trim_right(distance) else {
            return None
        };

        Some(this)
    }

    /// Extend the dimensions of a [`Rectangle`] by the specified distance on all edges.
    ///
    /// # Errors
    /// Will return if `None` if the new edges' coordinates flow are less than zero.
    #[must_use]
    pub const fn extend(self, distance: isize) -> Option<Self> {
        let Some(this) = self.trim_x(-distance) else {
            return None
        };

        let Some(this) = this.trim_y(-distance) else {
            return None
        };

        Some(this)
    }

    /// Get the (x,y) coordinates of the specified index.
    ///
    /// # Errors
    /// Will return `None` if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use tuit::terminal::Rectangle;
    ///
    /// let rect = Rectangle::of_size((20, 20));
    /// let (x, y) = rect.index_into(10).unwrap();
    ///
    /// assert_eq!(x, 10);
    /// assert_eq!(y, 0);
    ///
    /// let (x, y) = rect.index_into(25).unwrap();
    ///
    /// assert_eq!(x, 5);
    /// assert_eq!(y, 1);
    ///
    /// let (x, y) = rect.index_into(20).unwrap();
    ///
    /// assert_eq!(x, 0);
    /// assert_eq!(y, 1);
    /// ```
    #[must_use] pub const fn index_into(&self, index: usize) -> Option<(usize, usize)> {
        let (width, height) = self.dimensions();

        if index > width * height {
            return None
        }

        let x = index % width;
        let y = index / width;

        Some((x, y))
    }
}