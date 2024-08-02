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

pub use const_size::ConstantSize;
pub use const_size_ref::ConstantSizeRef;
pub use interactive::*;
pub use max_size::MaxSize;

use crate::prelude::*;
use crate::style::Style;
#[allow(unused_imports)]
use crate::terminal;
use crate::terminal::view::View;

/// Module containing all the code required for the "interactive" aspects of Tuit. This includes code
/// like structs for handling input, like [`interactive::MouseButton`] or [`interactive::KeyState`].
pub mod interactive;


/// Code for the [`ConstantSize`] terminal.
pub mod const_size;
/// Code for the [`ConstantSizeRef`] terminal.
pub mod const_size_ref;
/// Code for the [`MaxSize`] terminal.
pub mod max_size;


#[cfg(feature = "owo_colors")]
mod owo_colors;
mod dummy;
/// The [`View`] terminal, that can provide mutable or immutable views into terminals.
mod view;
mod view_iterator;

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
        Rectangle::of_size(self.width(), self.height())
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
    ///
    /// let mut my_terminal: ConstantSize<20, 20> = ConstantSize::new();
    ///
    /// let mut stdout = stdout();
    ///
    /// #[cfg(feature = "ansi_terminal")]
    /// my_terminal.display(stdout).expect("Failed to display the terminal");
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
    ///
    /// let mut terminal: ConstantSize<20, 20> = ConstantSize::new();
    ///
    /// let my_character_ref = terminal.cell_mut(0, 0).expect("There should always be a character here!");
    ///
    /// // Set the top-right character to 'h'.
    /// my_character_ref.character = 'h';
    ///
    /// // NOTE: You need to enable the "ansi_terminal" feature for Stdout to implement TerminalDisplayTarget
    /// let std_out = std::io::stdout();
    ///
    /// terminal.display(std_out).expect("Failed to display terminal");
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
    pub const fn of_size(width: usize, height: usize) -> Self {
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
            self.right_bottom.0 = new_edge;
            (self.right_bottom.0, self.left_top.0) = (self.left_top.0, self.right_bottom.0);
        }

        self
    }

    /// A method to set the left edge of the [`Rectangle`] to the specified x-coordinate.
    #[must_use]
    pub const fn left_to(mut self, new_edge: usize) -> Self {
        if new_edge <= self.left() {
            self.left_top.0 = new_edge;
        } else {
            self.left_top.0 = new_edge;
            (self.right_bottom.0, self.left_top.0) = (self.left_top.0, self.right_bottom.0);
        }

        self
    }

    /// A method to set the bottom edge of the [`Rectangle`] to the specified y-coordinate.
    #[must_use]
    pub const fn bottom_to(mut self, new_edge: usize) -> Self {
        if new_edge >= self.bottom() {
            self.right_bottom.1 = new_edge;
        } else {
            self.right_bottom.1 = new_edge;
            (self.left_top.1, self.right_bottom.1) = (self.right_bottom.1, self.left_top.1);
        }

        self
    }

    /// A method to set the top edge of the [`Rectangle`] to the specified y-coordinate.
    #[must_use]
    pub const fn top_to(mut self, new_edge: usize) -> Self {
        if new_edge <= self.top() {
            self.left_top.1 = new_edge;
        } else {
            self.left_top.1 = new_edge;
            (self.left_top.1, self.right_bottom.1) = (self.right_bottom.1, self.left_top.1);
        }

        self
    }

    /// Check if the given (x,y) coordinate is within the [`Rectangle`].
    ///
    /// ```
    /// use tuit::terminal::Rectangle;
    ///
    /// let rectangle = Rectangle::of_size(20, 20);
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
    /// let rectangle = Rectangle::of_size(20, 20);
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
    pub const fn to(mut self, new_left_top: (usize, usize)) -> Self {
        let width = self.width();
        let height = self.height();

        self.left_top = new_left_top;
        self.right_bottom = (self.left_top.0 + width, self.left_top.1 + height);

        self
    }
}