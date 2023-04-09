//! # Terminals
//!
//! This module encompasses the main traits needed to implement a Twinkle Terminal.

use core::borrow::BorrowMut;
use core::ops::BitOr;
use core::time::Duration;
use crate::Error;

/// Represents a 4-bit ANSI terminal colour.
///
/// Usually, two of these are used in a terminal to create an 8-bit colour consisting
/// of a foreground and a background.
///
/// Note: Ansi4 was decided upon instead of Ansi16 in order to avoid name collisions when importing
/// both twinkle::terminal::Ansi4 and twinkle::terminal::TerminalColours::*
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Default)]
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
    Gray = 7,
    DarkGray = 8,
    BrightRed = 9,
    BrightGreen = 10,
    BrightYellow = 11,
    BrightBlue = 12,
    BrightMagenta = 13,
    BrightCyan = 14,
    BrightWhite = 15
}

impl BitOr for Ansi4 {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u8 | (rhs as u8) << 4
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Default)]
/// These are the possible terminal colours covered by Twinkle.
///
///
/// `TerminalColour` supports many terminal colour schemes; some terminals may not be capable of
/// displaying certain colours, or may represent colours differently (for example, ANSI colours
/// may be overwritten by user themes on some terminals).
///
/// `TerminalColour` supports the following colour formats:
/// - 16-colour ANSI
/// - 256-colour ANSI
/// - 24-bit true colour
/// - Luma8 grayscale colouring
/// - None (uses the last set terminal colour)
///
/// How terminals will display unsupported colour formats is implementation-specific; as a common
/// rule of thumb though, they should never panic when they see an unimplemented colour.
#[non_exhaustive]
pub enum TerminalColour {
    /// True colour
    Rgb24(u8, u8, u8),
    /// 8-bit grayscale colours
    Luma8(u8),
    /// 16-colour ANSI terminal colours
    Ansi16(Ansi4),
    /// ANSI 256-colour terminal colours
    Ansi256(u8),
    /// Use the terminal's default colour
    TerminalDefault,
    #[default]
    /// Use the last colour by the terminal
    None,
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
/// use twinkle::terminal::{Ansi4, TerminalColour, TerminalStyle};
///
/// let default_style = TerminalStyle::new()
///     .fg_ansi4(Ansi4::Green)
///     .bg_ansi4(Ansi4::Blue);
///
/// ```
pub struct TerminalStyle {
    /// The foreground colour of the terminal cell
    pub fg_colour: Option<TerminalColour>,
    /// The background colour of the terminal cell
    pub bg_colour: Option<TerminalColour>,
    /// The font weight of the terminal cell
    pub font_weight: Option<u16>,
    /// Whether the terminal cell is underline or not
    pub underline: Option<bool>,
    /// Whether the background and foreground colours should be switched; primarily for use in
    /// single-colour terminals.
    pub invert: Option<bool>
}

impl TerminalStyle {
    /// Creates a new TerminalStyle with all fields set to `None`
    pub fn new() -> Self {
        Self::default()
    }

    ///
    pub fn fg(mut self, fg_colour: TerminalColour) -> Self {
        self.fg_colour = Some(fg_colour);

        self
    }

    /// Used to set the background colour of the terminal style.
    ///
    /// ```
    /// use twinkle::terminal::{Ansi4, TerminalColour, TerminalStyle};
    ///
    /// // Note: there are individual methods for every TerminalColour variant,
    /// // but if you need to determine change the colour at runtime this method may be useful.
    /// let red_bg_style = TerminalStyle::new()
    ///                         // You can use `TerminalColour::bg_ansi4` here.
    ///                         .bg(TerminalColour::Ansi16(Ansi4::Black));
    /// ```
    pub fn bg(mut self, bg_colour: TerminalColour) -> Self {
        self.bg_colour = Some(bg_colour);

        self
    }

    /// Used to set the background colour of the terminal style to a 4-bit ANSI colour.
    ///
    /// ```
    /// use twinkle::terminal::{Ansi4, TerminalStyle};
    ///
    /// let blue_bg_style = TerminalStyle::new()
    ///                             .bg_ansi4(Ansi4::Blue);
    /// ```
    pub fn bg_ansi4(self, bg_colour: Ansi4) -> Self {
        self.bg(TerminalColour::Ansi16(bg_colour))
    }

    /// Used to set the foreground colour of the terminal style to a 4-bit ANSI colour.
    ///
    /// ```
    /// use twinkle::terminal::{Ansi4, TerminalStyle};
    ///
    /// let blue_fg_style = TerminalStyle::new()
    ///                             .fg_ansi4(Ansi4::Blue);
    /// ```
    pub fn fg_ansi4(self, fg_colour: Ansi4) -> Self {
        self.fg(TerminalColour::Ansi16(fg_colour))
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Default)]
/// This struct represents a character in the terminal (as well as all the styling that it may have)
pub struct TerminalCell {
    /// The character inside the cell
    pub character: char,
    /// The character's styling.
    pub style: TerminalStyle
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
/// This enum represents the various buttons on the mouse.
pub enum MouseButton {
    /// The left click button
    LeftClick,
    /// The right click button
    RightClick,
    /// Any auxiliary mouse buttons (for example, additional side buttons).
    AuxiliaryButton(u16)
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
/// KeyState encompasses the current states that a keyboard key can be in (just pressed, currently held, and just released)
pub enum KeyState {
    /// Key has just been pressed
    KeyDown,
    /// Key has just been released
    KeyUp,
    /// Key is currently held
    KeyHeld
}

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
/// `UpdateInfo` encapsulates the
pub enum UpdateInfo {
    /// This event triggers when a cell (character) gets clicked. It includes the X co-ordinate,
    /// Y co-ordinate, and the mouse button that was clicked.
    ///
    /// The variables are as follows: CellClicked(x_coord, y_coord, mouse_button)
    CellClicked(usize, usize, MouseButton),
    /// This can be sent to widgets to inform them of a printable keyboard key being
    /// pressed.
    KeyboardCharacter(char, KeyState),
    /// This can be sent to widgets to inform them of a keyboard key being pressed
    KeyboardInput(u16, KeyState),
    /// This can be used to inform widgets of how much time has passed since they have
    /// last been updated.
    TimeDelta(Duration),
}

/// This trait defines the minimum requirements for a type to be capable
///
/// ## Example
///
/// ```
/// use twinkle::prelude::Terminal;
/// use twinkle::terminal::{TerminalObject, UpdateInfo};
///
/// struct MyObject {
///     my_char: char
/// }
///
/// impl TerminalObject for MyObject {
///     fn update(&mut self, update_info: UpdateInfo, terminal: impl Terminal) {
///         match update_info {
///             // Change my_char to the last key that was pressed
///             UpdateInfo::KeyboardCharacter(character,_) => { self.my_char = character }
///             // Don't worry about anything else :)
///             _ => {}
///         }
///     }
///
///     fn draw(&mut self, mut terminal: impl Terminal) {
///         // Set the terminal's top-left character to my_char.
///         terminal.character_mut(0, 0).map(|x| x.character = self.my_char);
///     }
/// }

pub trait TerminalObject {
    /// This method is called by the implementor once the terminal receives an update.
    ///
    /// ```
    fn update(&mut self, update_info: UpdateInfo, terminal: impl Terminal) -> crate::Result<()>;

    /// This method is called by the implementor whenever a frame redraw is requested.
    fn draw(&mut self, terminal: impl Terminal);
}

/// The Terminal trait allows TerminalObjects to interact and manipulate a terminal's characters.
///
/// At its core, it just provides information about the terminal and access to its cells.
pub trait Terminal {
    /// Returns the Terminal's dimensions.
    fn dimensions(&self) -> (usize, usize);

    /// Returns the Terminal's default style.
    fn default_style(&self) -> TerminalStyle;

    /// Returns a mutable reference to the terminal's characters
    fn characters_mut(&mut self) -> &mut [TerminalCell];

    /// Returns an immutable reference to the terminal's characters
    fn characters(&self) -> &[TerminalCell];

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

    /// Retrieves a mutable reference to a terminal cell
    ///
    /// ```
    /// use twinkle::terminal::{ConstantSizeTerminal, Terminal};
    ///
    /// let mut terminal: ConstantSizeTerminal<20, 20> = ConstantSizeTerminal::new();
    ///
    /// let my_character_ref = terminal.character_mut(0, 0).expect("There should always be a character here!");
    ///
    /// // Set the top-right character to 'h'.
    /// my_character_ref.character = 'h';
    ///
    /// terminal.display(std_out);
    /// ```
    fn character_mut(&mut self, x: usize, y: usize) -> Option<&mut TerminalCell> {
        let width = self.width();

        self.characters_mut().get_mut((width * y) + x)
    }

    /// Retrieves an immutable reference to a terminal cell
    fn character(&self, x: usize, y: usize) -> Option<&TerminalCell> {
        let width = self.width();

        self.characters().get((width * y) + x)
    }
}

impl<T: Terminal> TerminalExtended for T {}

/// **All types that implement Terminal will automatically implement TerminalExtended**
/// <br /> <br />
/// This trait contains methods that take generics, and as such, would make it impossible to turn
/// types that implement Terminal into a trait object. For certain use-cases, you may want to turn
/// your terminal objects into trait objects, so therefore TerminalExtended has been broken off into
/// its own trait that accepts generics.
pub trait TerminalExtended: Terminal {
    /// This method returns an immutable reference to the TerminalCells within the specified area.
    ///
    /// It requires that you provide the collection to collect into yourself, since the library
    /// cannot use allocators to create a collection like Vec for you.
    fn view<'a>(&'a self, x_offset: usize, y_offset: usize, height: usize, width: usize, collection: &'a mut impl Extend<&'_ TerminalCell>) -> crate::Result<()>
    where Self: 'a
    {
        let characters = self.characters();
        let (terminal_height, terminal_width) = self.dimensions();

        if y_offset+height > terminal_height {
            return Err(Error::OutOfBoundsCoordinate(x_offset+width, y_offset+height))
        }

        if x_offset+width > terminal_width {
            return Err(Error::OutOfBoundsCoordinate(x_offset+width, y_offset+height))
        }

        let view = (0..height).flat_map(|y| {
            let start = x_offset + ((y_offset + y) * terminal_width);
            let end = start + width;

            characters[start..end].iter()
        });

        view.collect_into(collection);

        Ok(())
    }
}

/// A zero-allocation terminal of constant size. The terminal's size is determined at compile time,
/// and can't be changed at runtime.
///
/// ```
/// use twinkle::terminal::ConstantSizeTerminal;
///
/// let mut terminal: ConstantSizeTerminal<20, 20> = ConstantSizeTerminal::new();
///
/// let widget: Prompt = Prompt::new("Hello world!");
///
/// widget.draw(&mut terminal);
///
/// let std_out = std::io::stdout();
///
/// terminal.display(std_out)
/// ```
pub struct ConstantSizeTerminal<const WIDTH: usize, const HEIGHT: usize> {
    characters: [[TerminalCell; WIDTH]; HEIGHT],
    default_style: TerminalStyle
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for ConstantSizeTerminal<WIDTH, HEIGHT> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> ConstantSizeTerminal<WIDTH, HEIGHT> {
    /// Creates a new ConstantSizeTerminal.
    ///
    /// Note that since this terminal uses const generics, the terminal's size must be defined within
    /// the type, not the function!
    ///
    /// ## Example
    ///
    /// ```
    /// use twinkle::terminal::ConstantSizeTerminal;
    ///
    /// let my_terminal: ConstantSizeTerminal<20, 20> = ConstantSizeTerminal::new(); // does not require arguments.
    /// ```
    pub fn new() -> Self {
        ConstantSizeTerminal {
            characters: core::array::from_fn(|_| core::array::from_fn(|_| TerminalCell {
                character: ' ',
                style: TerminalStyle::default(),
            })),
            default_style: Default::default()
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Terminal for ConstantSizeTerminal<WIDTH, HEIGHT> {
    fn dimensions(&self) -> (usize, usize) {
        (WIDTH, HEIGHT)
    }

    fn default_style(&self) -> TerminalStyle {
        self.default_style
    }

    fn characters_mut(&mut self) -> &mut [TerminalCell] {
        self.characters.flatten_mut()
    }

    fn characters(&self) -> &[TerminalCell] {
        self.characters.flatten()
    }
}