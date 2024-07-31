//! # Widgets
//!
//! The widgets module includes the necessary traits for widgets

use crate::prelude::*;
use crate::terminal::{Terminal, UpdateInfo, UpdateResult};

#[cfg(feature = "builtin_widgets")]
/// Builtin widgets.
pub mod builtins;

/// Provides a direction for elements
pub enum Direction {
    /// Left
    Left,
    /// Right
    Right,
    /// Down (or bottom of terminal, in some contexts)
    Down,
    /// Up (or top of terminal, in some contexts)
    Up,
}

/// This trait defines the minimum requirements for a type to be capable of terminal display
///
/// ## Example
///
/// ```
/// use tuit::prelude::{Terminal, TerminalConst};
/// use tuit::terminal::{UpdateInfo, UpdateResult, TerminalMut};///
///
/// use tuit::widgets::Widget;
///
/// // Replaces the entire terminal with `my_char` on draw.
/// struct MyObject {
///     my_char: char
/// }
///
/// impl Widget for MyObject {
///     fn update(&mut self, update_info: UpdateInfo, terminal: impl TerminalConst) -> tuit::Result<UpdateResult> {
///         match update_info {
///             // Change my_char to the last key that was pressed
///             UpdateInfo::KeyboardCharacter(character,_) => { self.my_char = character }
///             // Don't worry about anything else :)
///             _ => {}
///         }
///
///         Ok(UpdateResult::NoEvent)
///     }
///
///     fn draw(&self, update_info: UpdateInfo, mut terminal: impl Terminal)-> tuit::Result<UpdateResult> {
///         // Set the terminal's top-left character to my_char.
///         terminal.character_mut(0, 0).map(|x| x.character = self.my_char);
///
///         Ok(UpdateResult::NoEvent)
///     }
/// }
/// ```
pub trait Widget {
    /// This method is called by the implementor once the terminal receives an update.
    ///
    /// ```
    /// # pub struct MyObject;
    /// # impl Widget for MyObject {
    /// #     fn update(&mut self, update_info: UpdateInfo, terminal: impl TerminalConst) -> tuit::Result<UpdateResult> {
    /// #         Ok(UpdateResult::NoEvent)
    /// #     }
    /// #     fn draw(&self, update_info: UpdateInfo, terminal: impl Terminal) -> tuit::Result<UpdateResult> {
    /// #         Ok(UpdateResult::NoEvent)
    /// #     }
    /// # }
    /// # fn await_input() -> UpdateInfo { UpdateInfo::KeyboardCharacter('x', KeyState::KeyDown) }
    ///
    /// use tuit::prelude::{Terminal, TerminalConst, Widget};
    /// use tuit::terminal::{ConstantSize, KeyState, UpdateInfo, UpdateResult};
    ///
    /// let mut  my_terminal: ConstantSize<20, 20> = ConstantSize::new();
    /// let mut my_terminal_object = MyObject;    ///
    ///
    /// let mut input: UpdateInfo = await_input();
    ///
    /// while input == UpdateInfo::NoInfo {
    ///     input = await_input();
    /// }
    ///
    /// my_terminal_object.update(input, &my_terminal).ok();
    /// ```
    ///
    /// # Errors
    ///
    /// The function will return an [`Err`] when the *widget* that is being updated experiences an error.
    fn update(
        &mut self,
        update_info: UpdateInfo,
        terminal: impl TerminalConst,
    ) -> crate::Result<UpdateResult>;

    /// This method is called by the implementor whenever a widget redraw is requested.
    ///
    /// # Errors
    ///
    /// This will return an [`Err`] if the [`Widget`] fails to correctly draw itself. The underlying error may
    /// provide more information on why the failure occurred.
    fn draw(&self, update_info: UpdateInfo, terminal: impl Terminal)
            -> crate::Result<UpdateResult>;

    //      NOTE: There was a "ForceRedraw" enum variant for [`UpdateInfo`] that has been removed
    //              because widgets should be expected to draw on every redraw call. Optimizing
    //              when to do draw calls is a detail for the implementor to handle.
    // /// This method is called by the implementor when a force redraw is required.
    // ///
    // /// Equivalent to [`Widget::draw`] when called with [`UpdateInfo::ForceRedraw`] as `update_info`.
    // fn force_redraw(&self, terminal: impl Terminal) -> crate::Result<UpdateResult> {
    //     self.draw(UpdateInfo::ForceRedraw, terminal)
    // }

    /// This method is called by the implementor when a redraw is requested.
    ///
    /// Equivalent to [`Widget::draw`] when called with [`UpdateInfo::NoInfo`] as `update_info`.
    ///
    /// Essentially a shorthand for `object.draw(UpdateInfo::NoInfo, my_terminal)`.
    ///
    /// # Errors
    ///
    /// This will return an [`Err`] if the [`Widget`] cannot redraw itself.
    fn drawn(&self, terminal: impl Terminal) -> crate::Result<UpdateResult> {
        self.draw(UpdateInfo::NoInfo, terminal)
    }
}

#[derive(
    Hash,
    Eq,
    PartialEq,
    Copy,
    Clone,
    Debug,
    Default
)] // Ord and PartialOrd also implemented. check default_impls.rs
/// Provides the edge coordinates for a tuit Square.
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
}

/// The [`BoundingBox`] trait allows widgets to show the area of the [`Terminal`] that they cover.
/// This is useful for optimizing draw calls by only redrawing the area in the [`BoundingBox`], and it's also
/// useful for composing widgets from other widgets because you can obtain data about the widget's
pub trait BoundingBox: Widget {
    /// Calculates the bounding box of the widget. This method is available so that other widgets can be
    /// composed using the [`BoundingBox`] widget, but it doesn't need to be used by the end-user of the [`Widget`].
    ///
    /// [`BoundingBox::bounding_box`] returns a [`Rectangle`] which contains the coordinates of the [`Rectangle`]'s edges and vertices.
    ///
    /// Keep in mind, the y-axis is flipped in Tuit, so [`Rectangle::bottom`] is actually the larger value,
    /// not [`Rectangle::top`]
    fn bounding_box(&self, terminal: impl TerminalConst) -> Rectangle;
    /// The [`BoundingBox::completely_covered`] method allows the widget to communicate whether it
    /// completely covers the space specified by its bounding box.
    ///
    /// For example, if a widget is circular, it will return [`false`] because it doesn't completely
    /// cover the space in its bounding box.
    fn completely_covered(&self, rectangle: Rectangle) -> bool;
}

#[cfg(test)]
#[doc(hidden)]
/// Proud to be a great programmer who tests his code. :)
mod test {}