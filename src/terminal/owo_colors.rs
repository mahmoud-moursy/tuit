use core::fmt::Formatter;

use owo_colors::{DynColor, DynColors, Effect, XtermColors};

use crate::style::{Ansi4, Colour, Style};

impl From<Ansi4> for owo_colors::AnsiColors {
    fn from(value: Ansi4) -> Self {
        match value {
            Ansi4::Black => Self::Black,
            Ansi4::Red => Self::Red,
            Ansi4::Green => Self::Green,
            Ansi4::Yellow => Self::Yellow,
            Ansi4::Blue => Self::Blue,
            Ansi4::Cyan => Self::Cyan,
            Ansi4::White => Self::White,
            Ansi4::BrightBlack => Self::BrightBlack,
            Ansi4::BrightRed => Self::BrightRed,
            Ansi4::BrightGreen => Self::BrightGreen,
            Ansi4::BrightYellow => Self::BrightYellow,
            Ansi4::BrightBlue => Self::BrightBlue,
            Ansi4::BrightCyan => Self::BrightCyan,
            Ansi4::BrightWhite => Self::BrightWhite,
            Ansi4::Magenta | Ansi4::BrightMagenta => Self::Magenta,
        }
    }
}

impl From<Colour> for DynColors {
    fn from(value: Colour) -> Self {
        match value {
            Colour::Rgb24(r, g, b) => Self::Rgb(r, g, b),
            Colour::Luma8(brightness) => Self::Rgb(brightness, brightness, brightness),
            Colour::Ansi16(fg) => Self::Ansi(fg.into()),
            Colour::Ansi256(fg) => Self::Xterm(XtermColors::from(fg)),
            Colour::TerminalDefault => Self::Ansi(owo_colors::AnsiColors::Default),
        }
    }
}

impl DynColor for Colour {
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

impl From<Style> for owo_colors::Style {
    #[allow(clippy::collapsible_else_if)]
    fn from(value: Style) -> Self {
        let Style {
            fg_colour,
            bg_colour,
            font_weight,
            underline,
            invert,
        } = value;

        let mut style = Self::new();

        if let Some(fg_colour) = fg_colour {
            let color: DynColors = fg_colour.into();

            if Some(true) == invert {
                style = style.on_color(color);
            } else {
                style = style.color(color);
            }
        } else {
            if Some(true) == invert {
                style = style.on_default_color();
            } else {
                style = style.default_color();
            }
        }

        if let Some(bg_colour) = bg_colour {
            let color: DynColors = bg_colour.into();

            if Some(true) == invert {
                style = style.color(color);
            } else {
                style = style.on_color(color);
            }
        } else {
            if Some(true) == invert {
                style = style.default_color();
            } else {
                style = style.on_default_color();
            }
        }

        if let Some(font_weight) = font_weight {
            if font_weight >= 700 {
                style = style.bold();
            } else {
                style = style.remove_effect(Effect::Bold);
            }
        }

        if let Some(underline) = underline {
            if underline {
                style = style.underline();
            } else {
                style = style.remove_effect(Effect::Underline);
            }
        }

        style
    }
}
