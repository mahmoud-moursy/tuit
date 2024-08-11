//! # Widgets
//!
//! The widgets module includes the necessary traits for widgets

use crate::prelude::*;
use crate::terminal::{Rectangle, Terminal, UpdateInfo, UpdateResult};

#[cfg(feature = "builtin_widgets")]
/// Builtin widgets.
pub mod builtins;

/// Provides a direction for [`Widget`]s to optionally use where it makes sense.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug, Hash)]
pub enum Direction {
    /// Left
    #[default]
    Left,
    /// Right
    Right,
    /// Down (or bottom of terminal, in some contexts)
    Down,
    /// Up (or top of terminal, in some contexts)
    Up,
}

impl Direction {
    /// Get an iterator of all the possible [`Direction`]s.
    pub fn enumerate() -> impl Iterator<Item = Self> {
        use Direction::{Down, Left, Right, Up};

        [Left, Right, Down, Up].into_iter()
    }
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
///         terminal.cell_mut(0, 0).map(|x| x.character = self.my_char);
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
    /// // Collecting input is not Tuit's responsibility -- so `await_input` here is an external function
    /// // created by a magical user of this library. 🧙
    /// let mut magical_input: UpdateInfo = await_input();
    ///
    /// while magical_input == UpdateInfo::NoInfo {
    ///     magical_input = await_input();
    /// }
    ///
    /// my_terminal_object.update(magical_input, &my_terminal).ok();
    /// ```
    ///
    /// # Errors
    ///
    /// The function will return an [`Err`] when the *[`Widget`]* that is being updated experiences an error.
    fn update(
        &mut self,
        update_info: UpdateInfo,
        terminal: impl TerminalConst,
    ) -> crate::Result<UpdateResult>;

    /// This method is called by the implementor whenever they want the widget to redraw.
    ///
    /// # Errors
    ///
    /// This will return an [`Err`] if the [`Widget`] fails to correctly draw itself. The underlying error may
    /// provide more information on why the failure occurred.
    /// 
    /// Assume that the terminal is in an invalid state when this function returns an [`Err`], so
    /// you should try redrawing again, perhaps without the widget that failed.
    fn draw(&self, update_info: UpdateInfo, terminal: impl Terminal)
            -> crate::Result<UpdateResult>;

    /// This method is called by the implementor when redrawing is requested.
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

/// The [`BoundingBox`] trait allows widgets to show the area of the [`Terminal`] that they cover.
/// This is useful for optimizing draw calls by only redrawing the area in the [`BoundingBox`],
/// and it's also useful for composing widgets from other widgets because you can collect data
/// about the widget's draw area.
pub trait BoundingBox: Widget {
    /// Calculates the bounding box of the widget. This method is available so that other widgets
    /// can be composed using the [`BoundingBox`] widget, but it doesn't need to be used by the
    /// end-user of the [`Widget`].
    ///
    /// It can potentially be used to optimize redrawing by only redrawing the bounding box.
    ///
    /// [`BoundingBox::bounding_box`] returns a [`Rectangle`] which contains the coordinates of
    /// the [`Rectangle`]'s edges and vertices.
    ///
    /// Keep in mind, the y-axis is flipped in Tuit, so [`Rectangle::bottom`] is actually the larger value,
    /// not [`Rectangle::top`].
    ///
    /// # Errors
    ///
    /// Sometimes it's not possible to return a valid [`Rectangle`] for the bounding box. Widgets should
    /// try their best to fit the bounds that they are given (e.g. by truncating text, etc.), but in case
    /// it is not possible to validly display the widget, the widget should return an [`Error`](crate::Error).
    fn bounding_box(&self, rect: Rectangle) -> crate::Result<Rectangle>;
    /// The [`BoundingBox::completely_covers`] method allows the widget to communicate whether it
    /// completely covers the space specified by the specified [`Rectangle`].
    ///
    /// For example, if a widget is circular, it will return [`false`] because it doesn't completely
    /// cover the space in the [`Rectangle`].
    fn completely_covers(&self, rectangle: Rectangle) -> bool;

    /// The [`BoundingBox::covered_in`] method allows the widget to communicate whether it
/// completely covers the space specified by its own bounding box in the specified
/// [`TerminalConst`].
///
/// For example, if the widget is circular, it will return [`false`] because it doesn't
/// completely cover the space in its bounding box.
///
/// # Parameters
///
/// - `terminal`: A reference to a [`TerminalConst`] instance that provides the necessary information about the terminal's size and dimensions.
///
/// # Return Value
///
/// This function returns a boolean value that indicates whether the widget completely covers the space specified by its own bounding box in the given [`TerminalConst`]. If the widget's bounding box cannot be determined or if it does not completely cover the space, the function returns [`false`]. Otherwise, it returns [`true`].

/// # Examples
///
/// ```
/// use tuit::widgets::{BoundingBox, Widget};
/// use tuit::terminal::{ConstantSize, Rectangle, TerminalConst};
/// use tuit::prelude::*;
///
/// use tuit::widgets::builtins::dummy::EmptyWidget;
///
/// // Create a widget that completely covers the terminal.
/// struct FullTerminalWidget;
///
/// impl EmptyWidget for FullTerminalWidget {}
///
/// impl BoundingBox for FullTerminalWidget {
///     fn bounding_box(&self, _rect: Rectangle) -> tuit::Result<Rectangle> {
///         Ok(Rectangle::new((0, 0), (20, 20)))
///     }
///
///     fn completely_covers(&self, _rectangle: Rectangle) -> bool {
///         // The widget completely covers the terminal, so it completely covers any rectangle.
///         true
///     }
/// }
///
/// let full_terminal_widget = FullTerminalWidget;
/// let terminal: ConstantSize<20, 20> = ConstantSize::new();
///
/// assert_eq!(full_terminal_widget.covered_in(&terminal), true);
/// ```
///
/// ```
/// use tuit::widgets::{BoundingBox, Widget};
/// use tuit::terminal::{ConstantSize, Rectangle, TerminalConst};
/// use tuit::prelude::*;
///
/// use tuit::widgets::builtins::dummy::EmptyWidget;
///
/// // Create a widget that does not completely cover the terminal.
/// struct PartialTerminalWidget;
///
/// impl EmptyWidget for PartialTerminalWidget {}
///
/// impl BoundingBox for PartialTerminalWidget {
///     fn bounding_box(&self, _rect: Rectangle) -> tuit::Result<Rectangle> {
///         Ok(Rectangle::new((0, 0), (10, 10)))
///     }
///
///     fn completely_covers(&self, _rectangle: Rectangle) -> bool {
///         // The widget is not rectangular in shape, so it does not completely cover any rectangle.
///         false
///     }
/// }
///
/// let partial_terminal_widget = PartialTerminalWidget;
/// let terminal: ConstantSize<20, 20> = ConstantSize::new();
///
/// assert_eq!(partial_terminal_widget.covered_in(&terminal), false);
/// ```
    fn covered_in(&self, terminal: impl TerminalConst) -> bool {
        let Ok(bounding_box) = self.bounding_box(terminal.bounding_box()) else {
            return false
        };

        self.completely_covers(bounding_box)
    }

    /// Get the bounding box of the widget in the specified [`Metadata`].
    ///
    /// This is a convenience method that calls [`BoundingBox::bounding_box`] with the [`Metadata::bounding_box`] of the specified terminal.
    ///
    /// # Errors
    ///
    /// This will return an [`Error`](crate::Error) if the [`BoundingBox::bounding_box`] method fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use tuit::widgets::{BoundingBox, Widget};
    /// use tuit::terminal::{ConstantSize, Rectangle, TerminalConst};
    /// use tuit::prelude::*;
    ///
    /// use tuit::widgets::builtins::dummy::EmptyWidget;
    ///
    /// // Create a widget that completely covers the terminal.
    /// struct FullTerminalWidget;
    ///
    /// impl EmptyWidget for FullTerminalWidget {}
    ///
    /// impl BoundingBox for FullTerminalWidget {
    ///     fn bounding_box(&self, rect: Rectangle) -> tuit::Result<Rectangle> {
    ///         Ok(Rectangle::new(rect.left_top(), rect.right_bottom())) // basically the same as passing `Ok(rect)`.
    ///     }
    ///
    ///     fn completely_covers(&self, _rectangle: Rectangle) -> bool {
    ///         // The widget always covers every rectangle without leaving any `Cell` untouched.
    ///         true
    ///     }
    /// }
    ///
    /// let full_terminal_widget = FullTerminalWidget;
    /// let terminal: ConstantSize<20, 20> = ConstantSize::new();
    ///
    /// assert_eq!(full_terminal_widget.bounding_box_in(&terminal).expect("Ok"), Rectangle::new((0, 0), (20, 20)));
    /// ```
    fn bounding_box_in(&self, terminal: impl Metadata) -> crate::Result<Rectangle> {
        self.bounding_box(terminal.bounding_box())
    }
}

#[cfg(test)]
#[doc(hidden)]
/// Proud to be a great programmer who tests his code. :)
mod test {}