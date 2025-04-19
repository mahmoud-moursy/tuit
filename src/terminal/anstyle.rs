//! Implements two-way conversion for most constructs from the [`anstyle`] crate.

use anstyle::AnsiColor as AnstyleAnsi;
use anstyle::Color as AnstyleColour;
use anstyle::Style as AnstyleStyle;

use crate::style::{Ansi4, Colour as TuitColour};
use crate::style::Style as TuitStyle;
use crate::style::Ansi4 as TuitAnsi4;

// Convert ANSI Colours
impl From<AnstyleAnsi> for TuitAnsi4 {
    fn from(value: AnstyleAnsi) -> Self {
        use crate::style::Ansi4;

        match value {
            AnstyleAnsi::Black => Ansi4::Black,
            AnstyleAnsi::Red => Ansi4::Red,
            AnstyleAnsi::Green => Ansi4::Green,
            AnstyleAnsi::Yellow => Ansi4::Yellow,
            AnstyleAnsi::Blue => Ansi4::Blue,
            AnstyleAnsi::Magenta => Ansi4::Magenta,
            AnstyleAnsi::Cyan => Ansi4::Cyan,
            AnstyleAnsi::White => Ansi4::White,
            AnstyleAnsi::BrightBlack => Ansi4::BrightBlack,
            AnstyleAnsi::BrightRed => Ansi4::BrightRed,
            AnstyleAnsi::BrightGreen => Ansi4::BrightGreen,
            AnstyleAnsi::BrightYellow => Ansi4::BrightYellow,
            AnstyleAnsi::BrightBlue => Ansi4::BrightBlue,
            AnstyleAnsi::BrightMagenta => Ansi4::BrightMagenta,
            AnstyleAnsi::BrightCyan => Ansi4::BrightCyan,
            AnstyleAnsi::BrightWhite => Ansi4::BrightWhite,
        }
    }
}

impl From<TuitAnsi4> for AnstyleAnsi {
    fn from(value: TuitAnsi4) -> Self {
        match value {
            Ansi4::Black => AnstyleAnsi::Black,
            Ansi4::Red => AnstyleAnsi::Red,
            Ansi4::Green => AnstyleAnsi::Green,
            Ansi4::Yellow => AnstyleAnsi::Yellow,
            Ansi4::Blue => AnstyleAnsi::Blue,
            Ansi4::Magenta => AnstyleAnsi::Magenta,
            Ansi4::Cyan => AnstyleAnsi::Cyan,
            Ansi4::White => AnstyleAnsi::White,
            Ansi4::BrightBlack => AnstyleAnsi::BrightBlack,
            Ansi4::BrightRed => AnstyleAnsi::BrightRed,
            Ansi4::BrightGreen => AnstyleAnsi::BrightGreen,
            Ansi4::BrightYellow => AnstyleAnsi::BrightYellow,
            Ansi4::BrightBlue => AnstyleAnsi::BrightBlue,
            Ansi4::BrightMagenta => AnstyleAnsi::BrightMagenta,
            Ansi4::BrightCyan => AnstyleAnsi::BrightCyan,
            Ansi4::BrightWhite => AnstyleAnsi::BrightWhite,
        }
    }
}

// Convert ful Colours
impl From<AnstyleColour> for TuitColour {
    fn from(value: AnstyleColour) -> Self {
        match value {
            AnstyleColour::Ansi(ansi) => {
                crate::style::Colour::Ansi16(ansi.into())
            }
            AnstyleColour::Ansi256(value) => {
                crate::style::Colour::Ansi256(value.0)
            }
            AnstyleColour::Rgb(value) => {
                crate::style::Colour::Rgb24(value.r(), value.g(), value.b())
            }
        }
    }
}

pub struct IncompatibleColour;

impl TryFrom<TuitColour> for AnstyleColour {
    type Error = IncompatibleColour;

    fn try_from(value: TuitColour) -> Result<Self, Self::Error> {
        match value {
            TuitColour::Rgb24(r, g, b) => {
                Ok(AnstyleColour::from((r, g, b)))
            }
            TuitColour::Luma8(luma) => {
                Ok(AnstyleColour::from((luma, luma, luma)))
            }
            TuitColour::Ansi16(ansi4) => {
                let anstyle_ansi = AnstyleAnsi::from(ansi4);
                Ok(AnstyleColour::Ansi(anstyle_ansi))
            }
            TuitColour::Ansi256(ansi256) => {
                let anstyle_ansi = anstyle::Ansi256Color(ansi256);
                Ok(AnstyleColour::Ansi256(anstyle_ansi))
            }
            TuitColour::TerminalDefault => {
                Err(IncompatibleColour)
            }
        }
    }
}

impl From<TuitColour> for Option<AnstyleColour> {
    fn from(value: TuitColour) -> Self {
        match value {
            TuitColour::TerminalDefault => None,
            otherwise => otherwise.try_into().unwrap()
        }
    }
}

// Convert Styles
impl TryFrom<AnstyleStyle> for TuitStyle {
    type Error = Self;

    fn try_from(value: AnstyleStyle) -> Result<Self, Self::Error> {
        let effects = value.get_effects();
        let underline = effects.contains(anstyle::Effects::UNDERLINE);
        let bold = effects.contains(anstyle::Effects::BOLD);
        let italic = effects.contains(anstyle::Effects::ITALIC);
        let strikethrough = effects.contains(anstyle::Effects::STRIKETHROUGH);
        let invert = effects.contains(anstyle::Effects::INVERT);

        let bg_colour = value.get_bg_color().map(TuitColour::from);
        let fg_colour = value.get_fg_color().map(TuitColour::from);

        // check if any Anstyle styling was lost.
        let lossy = effects.contains(anstyle::Effects::CURLY_UNDERLINE | anstyle::Effects::DOTTED_UNDERLINE | anstyle::Effects::DASHED_UNDERLINE | anstyle::Effects::DOUBLE_UNDERLINE);
        let lossy = lossy || effects.contains(anstyle::Effects::HIDDEN | anstyle::Effects::BLINK | anstyle::Effects::DIMMED);

        // Get the defined values for "boldness"
        let font_weight = if bold {
            TuitStyle::new().bold().font_weight
        } else {
            TuitStyle::new().regular().font_weight
        };

        let output = TuitStyle {
            fg_colour,
            bg_colour,
            font_weight,
            underline: Some(underline),
            invert: Some(invert),
            strikethrough: Some(strikethrough),
            italic: Some(italic)
        };

        if lossy {
            return Err(output)
        }

        Ok(output)
    }
}

impl From<TuitStyle> for AnstyleStyle {
    fn from(value: TuitStyle) -> Self {
        let TuitStyle {
            fg_colour,
            bg_colour,
            font_weight,
            underline,
            invert,
            strikethrough,
            italic
        } = value;

        let mut output = AnstyleStyle::new();

        if let Some(fg_colour) = fg_colour {
            output = output.fg_color(fg_colour.try_into().ok())
        }

        if let Some(bg_colour) = bg_colour {
            output = output.bg_color(bg_colour.try_into().ok());
        }

        if let Some(true) = underline {
            output = output.underline()
        }

        if let Some(true) = invert {
            output = output.invert();
        }

        if let Some(true) = strikethrough {
            output = output.strikethrough();
        }

        if let Some(font_weight) = font_weight {
            if font_weight >= 700 {
                output = output.bold();
            }
        }

        if let Some(true) = italic {
            output = output.italic();
        }

        output
    }
}

