//! All the important types for styling in Tuit. Contains structs like `Colour` and `Style`.

/// Represents a 4-bit ANSI terminal colour.
/// <br /> <br />
/// Usually, two of these are used in a terminal to create an 8-bit colour consisting
/// of a foreground and a background.
/// <br /> <br />
/// Note: Ansi4 was decided upon instead of Ansi16 to avoid name collisions when importing
/// both [`Ansi4`] and when glob-importing [`Colour`]
///
/// ```
/// use tuit::style::Ansi4;
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

/// These are the possible terminal colours covered by Tuit.
///
///
/// [`Colour`] supports many terminal colour schemes; some terminals may not be capable of
/// displaying certain colours, or may represent colours differently (for example, user themes
/// may change the ANSI colours to something else on some terminals).
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
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Default, PartialOrd)]
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
/// use tuit::style::{Ansi4, Colour, Style};
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
    /// Creates a new [`Style`] with all fields set to `None`.
    ///
    /// It is equivalent to [`Style::default`], but it's usable in const contexts because it isn't a trait function.
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
    /// use tuit::style::{Ansi4, Colour, Style};
    ///
    /// // Note: there are individual methods for every TerminalColour variant,
    /// // but if you need to determine the colour at runtime this method may be useful.
    /// let black_fg_style = Style::new()
    ///                         // You can use [`Style::fg_ansi4`] here, but we're leaving it like this for sake of demonstration.
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
    /// use tuit::style::{Ansi4, Colour, Style};
    ///
    /// // Note: there are individual methods for every TerminalColour variant,
    /// // but if you need to determine change the colour at runtime this method may be useful.
    /// let black_bg_style = Style::new()
    ///                         // You can use [`Style::bg_ansi4`] here, but we're leaving it like this for sake of demonstration.
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
    /// use tuit::style::{Ansi4, Style};
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
    /// use tuit::style::{Ansi4, Style};
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
    /// use tuit::style::Style;
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
    /// use tuit::style::Style;
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
    /// use tuit::style::Style;
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
    /// use tuit::style::Style;
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
    /// use tuit::style::Style;
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
    /// use tuit::style::Style;
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
    /// use tuit::style::Style;
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
    /// use tuit::style::Style;
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
    /// use tuit::style::Style;
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