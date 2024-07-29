//! # Terminals
//!
//! This module encompasses the main traits needed to implement a Tuit Terminal.
//!
//! ## Implement a Terminal.
//!
//! The [`TerminalConst`] trait requires 3 things.
//!
//! 1. That your characters be stored in a contiguous chunk in memory. If you use more advanced data structures, then
//!     you must still return an `&mut [Character]` slice.
//!
//! ```
//! pub struct BasicTerminal;
//! ```

use core::ops::BitOr;
use core::time::Duration;

pub use const_size::ConstantSize;
pub use const_size_ref::ConstantSizeRef;
pub use max_size::MaxSize;

use crate::prelude::*;

mod const_size;
mod const_size_ref;
mod default_impls;
pub mod extended;
mod max_size;
#[cfg(feature = "owo_colors")]
mod owo_colors;

/// Represents a 4-bit ANSI terminal colour.
/// <br /> <br />
/// Usually, two of these are used in a terminal to create an 8-bit colour consisting
/// of a foreground and a background.
/// <br /> <br />
/// Note: Ansi4 was decided upon instead of Ansi16 in order to avoid name collisions when importing
/// both [`Ansi4`] and when glob-importing [`Colour`]
///
/// ```
/// use tuit::terminal::Ansi4;
///
/// let my_foreground_colour = Ansi4::Black;
/// let my_background_colour = Ansi4::Blue;
///
/// let my_colour: u8 = my_foreground_colour | my_background_colour;
/// ```
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Default, PartialOrd)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum Ansi4 {
    #[default]
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
    BrightBlack = 8,
    BrightRed = 9,
    BrightGreen = 10,
    BrightYellow = 11,
    BrightBlue = 12,
    BrightMagenta = 13,
    BrightCyan = 14,
    BrightWhite = 15,
}

impl BitOr for Ansi4 {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        (self as u8) << 4 | rhs as u8
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Default, PartialOrd)]
/// These are the possible terminal colours covered by Tuit.
///
///
/// [`Colour`] supports many terminal colour schemes; some terminals may not be capable of
/// displaying certain colours, or may represent colours differently (for example, ANSI colours
/// may be overwritten by user themes on some terminals).
///
/// [`Colour`] supports the following colour formats:
/// - 16-colour ANSI
/// - 256-colour ANSI
/// - 24-bit true colour
/// - Luma8 grayscale colouring
/// - Default (Uses the terminal's default colour)
///
/// How terminals will display unsupported colour formats is implementation-specific; as a common
/// rule of thumb though, they should never panic when they see an unimplemented colour.
#[non_exhaustive]
pub enum Colour {
    /// True colour
    Rgb24(u8, u8, u8),
    /// 8-bit grayscale colours
    Luma8(u8),
    /// 16-colour ANSI terminal colours
    Ansi16(Ansi4),
    /// 256-colour ANSI terminal colours
    Ansi256(u8),
    #[default]
    /// Use the terminal's default colour
    TerminalDefault,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Default)]
/// This struct contains a cell's styling data.
/// If a field is set to none, it will use the data from the last cell in the terminal that had it set.
/// If a field is None for all cells, then it will assume the terminal default style.
///
/// The style data includes the font's weight, colour, and whether it is underlined or not. It also
/// includes information about whether the foreground and background colours are switched.
///
/// ```
/// use tuit::terminal::{Ansi4, Colour, Style};
///
/// let default_style = Style::new()
///     .fg_ansi4(Ansi4::Green)
///     .bg_ansi4(Ansi4::Blue);
///
/// ```
pub struct Style {
    /// The foreground colour of the terminal cell
    ///
    /// When it is None, assume the colour to be unset (use the colour of the preceding cell)
    pub fg_colour: Option<Colour>,
    /// The background colour of the terminal cell
    ///
    /// When it is None, assume the colour to be unset (use the colour of the preceding cell)
    pub bg_colour: Option<Colour>,
    /// The font weight of the terminal cell
    ///
    /// When it is None, assume the font weight to be unset (use the font weight of the preceding cell)
    pub font_weight: Option<u16>,
    /// Whether the terminal cell is underlined or not
    ///
    /// When it is None, assume the underline to be unset (use the underlining of the preceding cell)
    pub underline: Option<bool>,
    /// Whether the background and foreground colours should be switched; primarily for use in
    /// single-colour terminals.
    ///
    /// When it is None, assume the inversion to be unset (use the inversion setting of the preceding cell)
    pub invert: Option<bool>,
}

impl Style {
    /// Creates a new [`Style`] with all fields set to `None`
    ///
    /// It is equivalent to [`Style::default`].
    #[must_use]
    pub const fn new() -> Self {
        // We initialize all the values with `None` instead of using `Self::default()`, which is
        // equivalent so that this function can be used in a const context.
        Self {
            fg_colour: None,
            bg_colour: None,
            font_weight: None,
            underline: None,
            invert: None,
        }
    }

    /// Used to set the foreground colour of the terminal style.
    ///
    /// ```
    /// use tuit::terminal::{Ansi4, Colour, Style};
    ///
    /// // Note: there are individual methods for every TerminalColour variant,
    /// // but if you need to determine the colour at runtime this method may be useful.
    /// let black_fg_style = Style::new()
    ///                         // You can use [`TerminalColour::fg_ansi4`] here, but we're leaving it like this for sake of demonstration.
    ///                         .fg(Colour::Ansi16(Ansi4::Black));
    /// ```
    #[must_use]
    pub const fn fg(mut self, fg_colour: Colour) -> Self {
        self.fg_colour = Some(fg_colour);

        self
    }

    /// Used to set the background colour of the terminal style.
    ///
    /// ```
    /// use tuit::terminal::{Ansi4, Colour, Style};
    ///
    /// // Note: there are individual methods for every TerminalColour variant,
    /// // but if you need to determine change the colour at runtime this method may be useful.
    /// let black_bg_style = Style::new()
    ///                         // You can use `TerminalColour::bg_ansi4` here.
    ///                         .bg(Colour::Ansi16(Ansi4::Black));
    /// ```
    #[must_use]
    pub const fn bg(mut self, bg_colour: Colour) -> Self {
        self.bg_colour = Some(bg_colour);

        self
    }

    /// Used to set the background colour of the terminal style to a 4-bit ANSI colour.
    ///
    /// ```
    /// use tuit::terminal::{Ansi4, Style};
    ///
    /// let blue_bg_style = Style::new()
    ///                             .bg_ansi4(Ansi4::Blue);
    /// ```
    #[must_use]
    pub const fn bg_ansi4(self, bg_colour: Ansi4) -> Self {
        self.bg(Colour::Ansi16(bg_colour))
    }

    /// Used to set the foreground colour of the terminal style to a 4-bit ANSI colour.
    ///
    /// ```
    /// use tuit::terminal::{Ansi4, Style};
    ///
    /// let blue_fg_style = Style::new()
    ///                             .fg_ansi4(Ansi4::Blue);
    /// ```
    #[must_use]
    pub const fn fg_ansi4(self, fg_colour: Ansi4) -> Self {
        self.fg(Colour::Ansi16(fg_colour))
    }

    /// Used to set the foreground colour of the terminal style to an 8-bit ANSI colour.
    ///
    /// ```
    /// use tuit::terminal::Style;
    ///
    /// let grayish_fg_style = Style::new()
    ///                             .fg_ansi8(10);
    /// ```
    #[must_use]
    pub const fn fg_ansi8(self, fg_colour: u8) -> Self {
        self.fg(Colour::Ansi256(fg_colour))
    }

    /// Used to set the background colour of the terminal style to an 8-bit ANSI colour.
    ///
    /// ```
    /// use tuit::terminal::Style;
    ///
    /// let grayish_bg_style = Style::new()
    ///                             .bg_ansi8(10);
    /// ```
    #[must_use]
    pub const fn bg_ansi8(self, bg_colour: u8) -> Self {
        self.bg(Colour::Ansi256(bg_colour))
    }

    /// Used to set the background colour of the terminal style to an 8-bit luminosity.
    ///
    /// ```
    /// use tuit::terminal::Style;
    ///
    /// let grayish_bg_style = Style::new()
    ///                             .bg_luma8(100);
    /// ```
    #[must_use]
    pub const fn bg_luma8(self, bg_luminosity: u8) -> Self {
        self.bg(Colour::Luma8(bg_luminosity))
    }

    /// Used to set the background colour of the terminal style to an 8-bit luminosity.
    ///
    /// ```
    /// use tuit::terminal::Style;
    ///
    /// let grayish_fg_style = Style::new()
    ///                             .fg_luma8(100);
    /// ```
    #[must_use]
    pub const fn fg_luma8(self, fg_luminosity: u8) -> Self {
        self.fg(Colour::Luma8(fg_luminosity))
    }

    /// Used to set the background colour of the terminal style to an 8-bit luminosity.
    ///
    /// ```
    /// use tuit::terminal::Style;
    ///
    /// let grayish_bg_style = Style::new()
    ///                             .bg_rgb24(100, 100, 100);
    /// ```
    #[must_use]
    pub const fn bg_rgb24(self, r: u8, g: u8, b: u8) -> Self {
        self.bg(Colour::Rgb24(r, g, b))
    }

    /// Used to set the background colour of the terminal style to an 8-bit luminosity.
    ///
    /// ```
    /// use tuit::terminal::Style;
    ///
    /// let grayish_fg_style = Style::new()
    ///                             .fg_rgb24(100, 100, 100);
    /// ```
    #[must_use]
    pub const fn fg_rgb24(self, r: u8, g: u8, b: u8) -> Self {
        self.fg(Colour::Rgb24(r, g, b))
    }

    /// Used to set the background colour of the terminal style to the default colour.
    ///
    /// ```
    /// use tuit::terminal::Style;
    ///
    /// let grayish_bg_style = Style::new()
    ///                             .bg_default();
    /// ```
    #[must_use]
    pub const fn bg_default(self) -> Self {
        self.bg(Colour::TerminalDefault)
    }

    /// Used to set the foreground colour of the terminal style to the default colour.
    ///
    /// ```
    /// use tuit::terminal::Style;
    ///
    /// let grayish_bg_style = Style::new()
    ///                             .fg_default();
    /// ```
    #[must_use]
    pub const fn fg_default(self) -> Self {
        self.fg(Colour::TerminalDefault)
    }

    /// Used to set the terminal style's underline to a user-specified value.
    #[must_use]
    pub const fn underline(mut self, underline: bool) -> Self {
        self.underline = Some(underline);

        self
    }

    /// Used to set the terminal style to "underlined".
    #[must_use]
    pub const fn underlined(mut self) -> Self {
        self.underline = Some(true);

        self
    }

    /// Used to set the terminal style to explicitly *not* underlined.
    #[must_use]
    pub const fn not_underlined(mut self) -> Self {
        self.underline = Some(false);

        self
    }

    /// Used to set the terminal style's font weight.
    #[must_use]
    pub const fn font_weight(mut self, weight: u16) -> Self {
        self.font_weight = Some(weight);

        self
    }

    /// Used to set the terminal style's inversion to a user-specified value.
    ///
    /// Refer to [`Style`] for an explanation on what inversion is.
    #[must_use]
    pub const fn inversion(mut self, inversion: bool) -> Self {
        self.invert = Some(inversion);

        self
    }

    /// Used to set the terminal style's inversion to specifically *true*.
    ///
    /// Refer to [`Style`] for an explanation on what inversion is.
    #[must_use]
    pub const fn inverted(mut self) -> Self {
        self.invert = Some(true);

        self
    }

    /// Used to set the terminal style's inversion to specifically *false*.
    ///
    /// Refer to [`Style`] for an explanation on what inversion is.
    #[must_use]
    pub const fn not_inverted(mut self) -> Self {
        self.invert = Some(false);

        self
    }

    /// Will replace all `None` properties in a style with defined properties from the right-hand style.
    ///
    /// ```
    /// use tuit::terminal::Style;
    /// let full_style = Style::new()
    ///                             .fg_ansi8(13)
    ///                             .bg_ansi8(255)
    ///                             .font_weight(30)
    ///                             .underlined()
    ///                             .inverted();
    ///
    /// let partial_style = Style::new()
    ///                             .fg_ansi8(12)
    ///                             .bg_ansi8(128);
    ///
    /// let new_style = partial_style.inherits(full_style);
    ///
    /// assert_eq!(
    ///     new_style,
    ///     // Pre-existing styles preserved, but those that were `None` got filled in.
    ///     Style::new().fg_ansi8(12).bg_ansi8(128).font_weight(30).underlined().inverted()
    /// );
    ///
    /// ```
    #[must_use]
    pub const fn inherits(self, fallback: Self) -> Self {
        // This macro is necessary because Rust's core lib does not support using `.or()` on `Option`s in a const context at the moment.
        macro_rules! or {
            ($lhs: expr, $rhs: expr) => {
                match $lhs {
                    Some(_) => $lhs,
                    None => $rhs,
                }
            };
        }

        Self {
            fg_colour: or!(self.fg_colour, fallback.fg_colour),
            bg_colour: or!(self.bg_colour, fallback.bg_colour),
            font_weight: or!(self.font_weight, fallback.font_weight),
            underline: or!(self.underline, fallback.underline),
            invert: or!(self.invert, fallback.invert),
        }
    }
}

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

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
/// This enum represents the various buttons on the mouse.
pub enum MouseButton {
    /// The left click button
    LeftClick,
    /// The right click button
    RightClick,
    /// Any auxiliary mouse buttons (for example, additional side buttons).
    AuxiliaryButton(u16),
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
/// [`KeyState`] encompasses the current states that a keyboard key can be in (just pressed, currently held, and just released)
pub enum KeyState {
    /// Key has just been pressed
    KeyDown,
    /// Key has just been released
    KeyUp,
    /// Key is currently held
    KeyHeld,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// `UpdateInfo` encapsulates the information sent after an update
pub enum UpdateInfo {
    /// This event triggers when a cell (character) gets clicked. It includes the X co-ordinate,
    /// Y co-ordinate, and the mouse button that was clicked.
    ///
    /// <br>
    /// The variables are as follows:
    /// `CellClicked(x_coord, y_coord, mouse_button)`
    CellClicked(usize, usize, MouseButton),
    /// This can be sent to widgets to inform them of a printable keyboard key being
    /// pressed.
    KeyboardCharacter(char, KeyState),
    /// This can be sent to widgets to inform them of a non-printable keyboard key being pressed.
    ///
    /// Go to [the UEFI specification](https://uefi.org/specs/UEFI/2.10/Apx_B_Console.html) and
    /// check for USB keyboard HID values.
    KeyboardInput(u8, KeyState),
    /// This can be used to inform widgets of how much time has passed since they have
    /// last been updated.
    TimeDelta(Duration),
    /// This is used to inform widgets that the terminal has been resized so that they can
    /// re-calculate their dimensions or any other cached data reliant on the terminal's size.
    TerminalResized,
    /// This is used when there is no information to report to widgets that need to be updated.
    NoInfo,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// `UpdateResult` includes information about the object's status. Objects are unable to destruct themselves,
/// and thus are reliant on the implementor to get rid of them once their lifecycle is over.
pub enum UpdateResult {
    /// No event has occurred, the object will continue to live.
    NoEvent,
    /// The object will continue to live, and it has not changed anything about how it draws itself.
    NoRedraw,
    /// The object's lifecycle has ended, and it should now be destructured.
    LifecycleEnd,
}

/// This trait defines the minimum requirements for a type to be capable of terminal display
///
/// ## Example
///
/// ```
/// use tuit::prelude::TerminalConst;
/// use tuit::terminal::{Widget, UpdateInfo, UpdateResult, TerminalMut};
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
///     fn draw(&mut self, update_info: UpdateInfo, mut terminal: impl TerminalConst)-> tuit::Result<UpdateResult> {
///         // Set the terminal's top-left character to my_char.
///         terminal.character_mut(0, 0).map(|x| x.character = self.my_char);
///
///         Ok(UpdateResult::NoEvent)
///     }
/// }
pub trait Widget {
    /// This method is called by the implementor once the terminal receives an update.
    ///
    /// ```no_test
    /// use tuit::terminal::{ConstantSizeTerminal, UpdateInfo};
    ///
    /// let mut  my_terminal: ConstantSizeTerminal<20, 20> = ConstantSizeTerminal::new();
    /// let my_terminal_object = MyObject;
    ///
    /// loop {
    ///     let input_magic: UpdateInfo = await_input();
    ///
    ///     my_terminal_object.update(input_magic, &mut my_terminal)
    /// }
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
    /// This will return an [`Err`] if the [`Widget`] fails to correctly draw itself. The underlying [`Error`] may
    /// provide more information on why that is.
    fn draw(&self, update_info: UpdateInfo, terminal: impl Terminal)
        -> crate::Result<UpdateResult>;

    //      NOTE: There was a "ForceRedraw" enum variant for [`UpdateInfo`] that has been removed
    //              because widgets should be expected to draw on every redraw call. Optimizing
    //              draw calls is a detail for the implementor to handle.
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

#[allow(clippy::module_name_repetitions)]
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
}

/// The [`TerminalConst`] trait allows [`Widget`]s to view a terminal's characters and style properties.
///
/// At its core, it just provides information about the terminal and const-only access to its characters.
#[allow(clippy::module_name_repetitions)]
pub trait TerminalConst: Metadata {
    //
    // /// Returns a mutable reference to the terminal's characters
    // fn characters_slice_mut(&mut self) -> &mut [Cell];

    /// Returns an immutable reference to the terminal's characters
    fn characters_slice(&self) -> &[Cell];

    /// Retrieves an immutable reference to a terminal cell
    fn character(&self, x: usize, y: usize) -> Option<&Cell> {
        let (width, height) = self.dimensions();

        if x >= width || y >= height {
            return None;
        }

        self.characters_slice().get(x + (width * y))
    }

    /// You can pass any value that implements [`Target`] to get the terminal to update.
    ///
    /// Inversely, you can call [`Target::render`] on any Terminal and draw the screen
    ///
    /// ```compile_fail
    /// use tuit::terminal::ConstantSizeTerminal;
    /// use tuit::prelude::*;
    ///
    /// let mut  my_terminal: ConstantSizeTerminal<20, 20> = ConstantSizeTerminal::new();
    ///
    /// let my_gpu = MyGpu;
    ///
    /// my_terminal.display(&mut my_gpu).expect("Failed to display the terminal");
    /// ```
    ///
    /// # Errors
    ///
    /// This will fail when the [`Target`] implementor experiences aa problem rendering
    fn display(&self, mut display: impl Target) -> crate::Result<()>
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
    fn characters_slice_mut(&mut self) -> &mut [Cell];

    /// Retrieves a mutable reference to a terminal character
    ///
    /// ```feature="ansi_terminal"
    /// use tuit::terminal::ConstantSize;
    /// use tuit::prelude::*;
    ///
    /// let mut terminal: ConstantSize<20, 20> = ConstantSize::new();
    ///
    /// let my_character_ref = terminal.character_mut(0, 0).expect("There should always be a character here!");
    ///
    /// // Set the top-right character to 'h'.
    /// my_character_ref.character = 'h';
    ///
    /// // NOTE: You need to enable the "ansi_terminal" feature for Stdout to implement TerminalDisplayTarget
    /// let std_out = std::io::stdout();
    ///
    /// terminal.display(std_out).expect("Failed to display terminal");
    /// ```
    fn character_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        let (width, height) = self.dimensions();

        if x >= width || y >= height {
            return None;
        }

        self.characters_slice_mut().get_mut((width * y) + x)
    }
}

/// This trait combines both [`TerminalMut`] and [`TerminalConst`] and is auto-implemented for any
/// type that implements both.
pub trait Terminal: TerminalConst + TerminalMut + Metadata {}

impl<T: TerminalConst + TerminalMut + Metadata> Terminal for T {}
