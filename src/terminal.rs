//! # Terminals
//!
//! This module encompasses the main traits needed to implement a Tuit Terminal.

use core::array;
use core::fmt::Formatter;
use core::ops::{BitOr, DerefMut};
use core::time::Duration;

use owo_colors::{DynColor, DynColors, Effect, OwoColorize, XtermColors};

use crate::Error;
use crate::prelude::*;

/// Represents a 4-bit ANSI terminal colour.
/// <br /> <br />
/// Usually, two of these are used in a terminal to create an 8-bit colour consisting
/// of a foreground and a background.
/// <br /> <br />
/// Note: Ansi4 was decided upon instead of Ansi16 in order to avoid name collisions when importing
/// both [`Ansi4`] and when glob-importing [`TerminalColour`]
///
/// ```
/// use tuit::terminal::Ansi4;
///
/// let my_foreground_colour = Ansi4::Black;
/// let my_background_colour = Ansi4::Blue;
///
/// let my_colour: u8 = my_foreground_colour | my_background_colour;
/// ```
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

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Default)]
/// These are the possible terminal colours covered by Tuit.
///
///
/// [`TerminalColour`] supports many terminal colour schemes; some terminals may not be capable of
/// displaying certain colours, or may represent colours differently (for example, ANSI colours
/// may be overwritten by user themes on some terminals).
///
/// [`TerminalColour`] supports the following colour formats:
/// - 16-colour ANSI
/// - 256-colour ANSI
/// - 24-bit true colour
/// - Luma8 grayscale colouring
/// - Default (Uses the terminal's default colour)
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
/// use tuit::terminal::{Ansi4, TerminalColour, TerminalStyle};
///
/// let default_style = TerminalStyle::new()
///     .fg_ansi4(Ansi4::Green)
///     .bg_ansi4(Ansi4::Blue);
///
/// ```
pub struct TerminalStyle {
    /// The foreground colour of the terminal cell
    /// 
    /// When it is None, assume the colour to be unset (use the colour of the preceding cell)
    pub fg_colour: Option<TerminalColour>,
    /// The background colour of the terminal cell
    /// 
    /// When it is None, assume the colour to be unset (use the colour of the preceding cell)
    pub bg_colour: Option<TerminalColour>,
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
    pub invert: Option<bool>
}

impl TerminalStyle {
    /// Creates a new TerminalStyle with all fields set to `None`
    pub fn new() -> Self {
        Self::default()
    }

    /// Used to set the foreground colour of the terminal style.
    ///
    /// ```
    /// use tuit::terminal::{Ansi4, TerminalColour, TerminalStyle};
    ///
    /// // Note: there are individual methods for every TerminalColour variant,
    /// // but if you need to determine the colour at runtime this method may be useful.
    /// let black_fg_style = TerminalStyle::new()
    ///                         // You can use [`TerminalColour::fg_ansi4`] here, but we're leaving it like this for sake of demonstration.
    ///                         .fg(TerminalColour::Ansi16(Ansi4::Black));
    /// ```
    pub fn fg(mut self, fg_colour: TerminalColour) -> Self {
        self.fg_colour = Some(fg_colour);

        self
    }

    /// Used to set the background colour of the terminal style.
    ///
    /// ```
    /// use tuit::terminal::{Ansi4, TerminalColour, TerminalStyle};
    ///
    /// // Note: there are individual methods for every TerminalColour variant,
    /// // but if you need to determine change the colour at runtime this method may be useful.
    /// let black_bg_style = TerminalStyle::new()
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
    /// use tuit::terminal::{Ansi4, TerminalStyle};
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
    /// use tuit::terminal::{Ansi4, TerminalStyle};
    ///
    /// let blue_fg_style = TerminalStyle::new()
    ///                             .fg_ansi4(Ansi4::Blue);
    /// ```
    pub fn fg_ansi4(self, fg_colour: Ansi4) -> Self {
        self.fg(TerminalColour::Ansi16(fg_colour))
    }

    /// Used to set the foreground colour of the terminal style to an 8-bit ANSI colour.
    ///
    /// ```
    /// use tuit::terminal::TerminalStyle;
    ///
    /// let grayish_fg_style = TerminalStyle::new()
    ///                             .fg_ansi8(10);
    /// ```
    pub fn fg_ansi8(self, fg_colour: u8) -> Self {
        self.fg(TerminalColour::Ansi256(fg_colour))
    }

    /// Used to set the foreground colour of the terminal style to an 8-bit ANSI colour.
    ///
    /// ```
    /// use tuit::terminal::TerminalStyle;
    ///
    /// let grayish_bg_style = TerminalStyle::new()
    ///                             .bg_ansi8(10);
    /// ```
    pub fn bg_ansi8(self, bg_colour: u8) -> Self {
        self.bg(TerminalColour::Ansi256(bg_colour))
    }

    pub fn bg_luma8(self, bg_luminosity: u8) -> Self {
        self.bg(TerminalColour::Luma8(bg_luminosity))
    }

    pub fn fg_luma8(self, fg_luminosity: u8) -> Self {
        self.fg(TerminalColour::Luma8(fg_luminosity))
    }

    pub fn bg_rgb24(self, r: u8, g: u8, b: u8) -> Self {
        self.bg(TerminalColour::Rgb24(r, g, b))
    }

    pub fn bg_default(self) -> Self {
        self.bg(TerminalColour::TerminalDefault)
    }

    pub fn fg_default(self) -> Self {
        self.fg(TerminalColour::TerminalDefault)
    }

    /// Used to set the terminal style to underlined.
    pub fn underlined(mut self) -> Self {
        self.underline = Some(true);

        self
    }

    /// Used to set the terminal style to explicitly *not* underlined.
    pub fn not_underlined(mut self) -> Self {
        self.underline = Some(false);

        self
    }

    /// Used to set the terminal style's weight.
    pub fn font_weight(mut self, weight: u16) -> Self {
        self.font_weight = Some(weight);

        self
    }

    /// Used to set the terminal style's inversion.
    pub fn inverted(mut self) -> Self {
        self.invert = Some(true);

        self
    }

    /// Used to set the terminal style to explicitly *not* inverted.
    /// 
    /// Refer to [TerminalStyle] for an explanation on what inversion is.
    pub fn not_inverted(mut self) -> Self {
        self.invert = Some(false);

        self
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Default)]
/// This struct represents a character in the terminal (as well as all the styling that it may have)
pub struct TerminalCell {
    /// The character inside the cell
    pub character: char,
    /// The character's styling.
    pub style: TerminalStyle,
}

#[cfg(feature = "ansi_terminal")]
impl core::fmt::Display for TerminalCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let owo_style: owo_colors::Style = self.style.into();

        write!(f, "{}", self.character.style(owo_style))
    }
}

impl From<Ansi4> for owo_colors::AnsiColors {
    fn from(value: Ansi4) -> Self {
        use owo_colors::AnsiColors;

        match value {
            Ansi4::Black => AnsiColors::Black,
            Ansi4::Red => AnsiColors::Red,
            Ansi4::Green => AnsiColors::Green,
            Ansi4::Yellow => AnsiColors::Yellow,
            Ansi4::Blue => AnsiColors::Blue,
            Ansi4::Magenta => AnsiColors::Magenta,
            Ansi4::Cyan => AnsiColors::Cyan,
            Ansi4::White => AnsiColors::White,
            Ansi4::BrightBlack => AnsiColors::BrightBlack,
            Ansi4::BrightRed => AnsiColors::BrightRed,
            Ansi4::BrightGreen => AnsiColors::BrightGreen,
            Ansi4::BrightYellow => AnsiColors::BrightYellow,
            Ansi4::BrightBlue => AnsiColors::BrightBlue,
            Ansi4::BrightMagenta => AnsiColors::Magenta,
            Ansi4::BrightCyan => AnsiColors::BrightCyan,
            Ansi4::BrightWhite => AnsiColors::BrightWhite
        }
    }
}

impl From<TerminalColour> for DynColors {
    fn from(value: TerminalColour) -> Self {
        match value {
            TerminalColour::Rgb24(r, g, b) => DynColors::Rgb(r, g, b),
            TerminalColour::Luma8(brightness) => DynColors::Rgb(brightness, brightness, brightness),
            TerminalColour::Ansi16(fg) => {
                DynColors::Ansi(fg.into())
            }
            TerminalColour::Ansi256(fg) => {
                DynColors::Xterm(XtermColors::from(fg))
            }
            TerminalColour::TerminalDefault => {
                DynColors::Ansi(owo_colors::AnsiColors::Default)
            }
        }
    }
}

impl DynColor for TerminalColour {
    fn fmt_ansi_fg(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let dyn_color: DynColors = (*self).into();

        dyn_color.fmt_ansi_fg(f)
    }

    fn fmt_ansi_bg(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let dyn_color: DynColors = (*self).into();

        dyn_color.fmt_ansi_bg(f)
    }

    fn fmt_raw_ansi_fg(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let dyn_color: DynColors = (*self).into();

        dyn_color.fmt_raw_ansi_bg(f)
    }

    fn fmt_raw_ansi_bg(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let dyn_color: DynColors = (*self).into();

        dyn_color.fmt_raw_ansi_bg(f)
    }

    fn get_dyncolors_fg(&self) -> DynColors {
        let dyn_color: DynColors = (*self).into();

        dyn_color
    }

    fn get_dyncolors_bg(&self) -> DynColors {
        let dyn_color: DynColors = (*self).into();

        dyn_color
    }
}

impl From<TerminalStyle> for owo_colors::Style {
    fn from(value: TerminalStyle) -> Self {
        let TerminalStyle {
            fg_colour,
            bg_colour,
            font_weight,
            underline,
            invert
        } = value;

        let mut style = owo_colors::Style::new();

        if let Some(fg_colour) = fg_colour {
            let color: Result<DynColors, _> = fg_colour.try_into();

            color.map(|res| style = style.color(res)).ok();
        }

        if let Some(bg_colour) = bg_colour {
            let color: Result<DynColors, _> = bg_colour.try_into();

            color.map(|res| style = style.on_color(res)).ok();
        }

        if let Some(font_weight) = font_weight {
            if font_weight >= 700 {
                style = style.bold()
            } else {
                style = style.remove_effect(Effect::Bold)
            }
        }

        if let Some(underline) = underline {
            if underline {
                style = style.underline()
            } else {
                style = style.remove_effect(Effect::Underline)
            }
        }

        if let Some(invert) = invert {
            if invert {
                style = style.blink();
            } else {
                style = style.remove_effect(Effect::Blink)
            }
        }

        style
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
    /// This can be sent to widgets to inform them of a keyboard key being pressed
    KeyboardInput(u16, KeyState),
    /// This can be used to inform widgets of how much time has passed since they have
    /// last been updated.
    TimeDelta(Duration),
    /// Some widgets may not need to update on every draw() call, unless the terminal has been resized.
    ///
    /// This is used to inform widgets that the terminal has been resized so that they can
    /// re-calculate their dimensions and any cached data reliant on the terminal's size.
    TerminalResized,
    /// Some widgets may *not* redraw themselves on every draw() call; if they need to do so
    /// immediately, then this can be used to inform them.
    ForceRedraw,
    /// This is used when there is no information to report to widgets that need to be updated
    /// (e.g in a redraw).
    NoInfo
}

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
/// `UpdateResult` includes information about the object's status. Objects are unable to destruct themselves,
/// and thus are reliant on the implementor to get rid of them once their lifecycle is over.
pub enum UpdateResult {
    /// No event has occurred, the object will continue to live.
    NoEvent,
    /// The object will continue to live, and it has not drawn anything to the screen.
    NoRedraw,
    /// The object's lifecycle has ended, and it should now be destructured.
    LifecycleEnd,
}

/// This trait defines the minimum requirements for a type to be capable of terminal display
///
/// ## Example
///
/// ```
/// use tuit::prelude::Terminal;
/// use tuit::terminal::{TerminalObject, UpdateInfo, UpdateResult};
///
/// // Replaces the entire terminal with `my_char` on draw.
/// struct MyObject {
///     my_char: char
/// }
///
/// impl TerminalObject for MyObject {
///     fn update(&mut self, update_info: UpdateInfo, terminal: impl Terminal) -> tuit::Result<UpdateResult> {
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
///     fn draw(&mut self, update_info: UpdateInfo, mut terminal: impl Terminal)-> tuit::Result<UpdateResult> {
///         // Set the terminal's top-left character to my_char.
///         terminal.character_mut(0, 0).map(|x| x.character = self.my_char);
///
///         Ok(UpdateResult::NoEvent)
///     }
/// }
pub trait TerminalObject {
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
    fn update(&mut self, update_info: UpdateInfo, terminal: impl Terminal) -> crate::Result<UpdateResult>;

    /// This method is called by the implementor whenever a frame redraw is requested.
    fn draw(&mut self, update_info: UpdateInfo, terminal: impl Terminal) -> crate::Result<UpdateResult>;

    /// This method is called by the implementor when a force redraw is required.
    ///
    /// Equivalent to [`TerminalObject::draw`] when called with [`UpdateInfo::ForceRedraw`] as `update_info`.
    fn force_redraw(&mut self, terminal: impl Terminal) -> crate::Result<UpdateResult> {
        self.draw(UpdateInfo::ForceRedraw, terminal)
    }

    /// This method is called by the implementor when a redraw is requested.
    ///
    /// Equivalent to [`TerminalObject::draw`] when called with [`UpdateInfo::NoInfo`] as `update_info`.
    ///
    /// Essentially a shorthand for `object.draw(UpdateInfo::NoInfo, my_terminal)`.
    fn drawn(&mut self, terminal: impl Terminal) -> crate::Result<UpdateResult> {
        self.draw(UpdateInfo::NoInfo, terminal)
    }
}

/// The Terminal trait allows TerminalObjects to interact and manipulate a terminal's characters.
///
/// At its core, it just provides information about the terminal and access to its cells.
pub trait Terminal {
    /// Returns the Terminal's dimensions in the order of (width, height).
    ///
    /// ```
    /// # use tuit::terminal::{ConstantSizeTerminal, Terminal};
    /// # let my_terminal: ConstantSizeTerminal<1, 1> = ConstantSizeTerminal::new();
    ///
    /// let (width, height) = my_terminal.dimensions();
    /// ```
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
    /// ```rust,feature="ansi_terminal"
    /// use tuit::terminal::{ConstantSizeTerminal};
    /// use tuit::prelude::*;
    ///
    /// let mut terminal: ConstantSizeTerminal<20, 20> = ConstantSizeTerminal::new();
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
    fn character_mut(&mut self, x: usize, y: usize) -> Option<&mut TerminalCell> {
        let width = self.width();

        self.characters_mut().get_mut((width * y) + x)
    }

    /// Retrieves an immutable reference to a terminal cell
    fn character(&self, x: usize, y: usize) -> Option<&TerminalCell> {
        let width = self.width();

        self.characters().get(x + (width * y))
    }

    /// You can pass any value that implements TerminalDrawTarget to get the terminal to update.
    ///
    /// Inversely, you can call TerminalDrawTarget::render on any Terminal and draw the screen
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
    fn display(&mut self, mut display: impl TerminalDrawTarget) -> crate::Result<()>
        where Self: Sized + 'static {
        display.render(self as &mut (dyn Terminal))
    }
}

impl<T: DerefMut<Target: Terminal>> Terminal for T {
    fn dimensions(&self) -> (usize, usize) {
        (**self).dimensions()
    }

    fn default_style(&self) -> TerminalStyle {
        (**self).default_style()
    }

    fn characters_mut(&mut self) -> &mut [TerminalCell] {
        (**self).characters_mut()
    }

    fn characters(&self) -> &[TerminalCell] {
        (**self).characters()
    }
}

/// **All types that implement Terminal will automatically implement TerminalExtended**
/// <br /> <br />
/// This trait contains methods that take generics, and as such, would make it impossible to turn
/// types that implement Terminal into a trait object. For certain use-cases, you may want to turn
/// your terminal objects into trait objects, so therefore TerminalExtended has been broken off into
/// its own trait that accepts generics.
pub trait TerminalExtended: Terminal {
    /// Returns a copied view to the TerminalCells within the specified area.
    ///
    /// ```
    /// use tuit::terminal::{ConstantSizeTerminal, TerminalCell};
    /// use tuit::prelude::*;
    ///
    /// let my_terminal: ConstantSizeTerminal<20, 20> = ConstantSizeTerminal::new();
    ///
    /// // A get a view of height 2 and width 2, at x-y coords (1,1).
    /// let cells: [[TerminalCell; 2]; 2] = my_terminal.copied_view::<2, 2>(1, 1).expect("This should never fail!");
    /// ```
    fn copied_view<const WIDTH: usize, const HEIGHT: usize>(&self, x_offset: usize, y_offset: usize) -> crate::Result<[[TerminalCell; WIDTH]; HEIGHT]> {
        let (terminal_height, terminal_width) = self.dimensions();
        let characters = self.characters();

        if y_offset + HEIGHT > terminal_height {
            return Err(Error::OutOfBoundsCoordinate(x_offset + WIDTH, y_offset + HEIGHT));
        }

        if x_offset + WIDTH > terminal_width {
            return Err(Error::OutOfBoundsCoordinate(x_offset + WIDTH, y_offset + HEIGHT));
        }

        Ok(array::from_fn(|y|
            array::from_fn(|x|
                characters[x_offset + x + (y_offset + y) * terminal_width]
            )
        )
        )
    }
}

impl<T: Terminal> TerminalExtended for T {}

/// A zero-allocation terminal of constant size. The terminal's size is determined at compile time,
/// and can't be changed at runtime.
///
/// ```no_test
/// use tuit::terminal::ConstantSizeTerminal;
/// use tuit::prelude::*;
///
/// let mut terminal: ConstantSizeTerminal<20, 20> = ConstantSizeTerminal::new();
///
/// let widget: CustomPrompt = CustomPrompt::new("Hello world!");
///
/// widget.draw(&mut terminal);
///
/// let std_out = std::io::stdout();
///
/// terminal.display(std_out).expect("Failed to draw terminal");
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
    /// use tuit::terminal::ConstantSizeTerminal;
    ///
    /// let my_terminal: ConstantSizeTerminal<20, 20> = ConstantSizeTerminal::new(); // does not require arguments.
    /// ```
    pub fn new() -> Self {
        ConstantSizeTerminal {
            characters: [[TerminalCell {
                character: ' ',
                style: TerminalStyle::default(),
            }; WIDTH]; HEIGHT],
            default_style: Default::default(),
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

/// A zero-allocation re-scalable terminal that allocates the maximum size that it can scale to.
pub struct MaxSizeTerminal<const MAX_WIDTH: usize, const MAX_HEIGHT: usize> {
    characters: [[TerminalCell; MAX_WIDTH]; MAX_HEIGHT],
    default_style: TerminalStyle,
    dimensions: (usize, usize),
}

impl<const MAX_WIDTH: usize, const MAX_HEIGHT: usize> Default for MaxSizeTerminal<MAX_WIDTH, MAX_HEIGHT> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const MAX_WIDTH: usize, const MAX_HEIGHT: usize> MaxSizeTerminal<MAX_WIDTH, MAX_HEIGHT> {
    /// Creates a new MaxSizeTerminal at its maximum size.
    pub fn new() -> Self {
        Self {
            characters: array::from_fn(|_| array::from_fn(|_| Default::default())),
            default_style: Default::default(),
            dimensions: (MAX_WIDTH, MAX_HEIGHT),
        }
    }

    /// Rescales the terminal if the new specified width and height are below the maximum limits.
    /// Otherwise, it will return an Err, containing the values put into it.
    ///
    /// It is wise to redraw the terminal after doing this, since MaxSizeTerminal will hide away
    /// currently characters from objects
    ///
    /// ```
    /// use tuit::terminal::MaxSizeTerminal;
    /// use tuit::prelude::*;
    ///
    /// let mut my_max_terminal: MaxSizeTerminal<20, 20> = MaxSizeTerminal::new();
    ///
    /// my_max_terminal.rescale(10, 10).expect("This mustn't fail!");
    ///
    /// let (overflowing_width, overflowing_height) = my_max_terminal.rescale(21, 10).expect_err("This must always be an error!");
    ///
    /// assert_eq!(overflowing_width, 21);
    /// assert_eq!(overflowing_height, 10);
    /// ```
    pub fn rescale(&mut self, new_width: usize, new_height: usize) -> Result<(), (usize, usize)> {
        if new_width > MAX_WIDTH {
            return Err((new_width, new_height))
        }

        if new_height > MAX_HEIGHT {
            return Err((new_width, new_height))
        }

        self.dimensions = (new_width, new_height);

        Ok(())
    }
}

impl<const MAX_WIDTH: usize, const MAX_HEIGHT: usize> Terminal for MaxSizeTerminal<MAX_WIDTH, MAX_HEIGHT> {
    fn dimensions(&self) -> (usize, usize) {
        self.dimensions
    }

    fn default_style(&self) -> TerminalStyle {
        self.default_style
    }

    fn characters_mut(&mut self) -> &mut [TerminalCell] {
        let acting_height = self.dimensions.1;
        let acting_width = self.dimensions.0;

        self.characters[0..acting_height][0..acting_width].flatten_mut()
    }

    fn characters(&self) -> &[TerminalCell] {
        let acting_height = self.dimensions.1;
        let acting_width = self.dimensions.0;

        self.characters[0..acting_height][0..acting_width].flatten()
    }
}