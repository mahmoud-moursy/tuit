#![doc(hidden)]

extern crate std;

use core::fmt::Formatter;

use owo_colors::OwoColorize;

use crate::draw::Renderer;
use crate::prelude::TerminalConst;
use crate::terminal::Cell;

#[cfg(feature = "ansi_terminal")]
impl Renderer for std::io::Stdout {
    fn render(&mut self, terminal: impl TerminalConst) -> crate::Result<()> {
        use std::io::Write;
        use std::prelude::rust_2021::*;

        let terminal_width = terminal.width();

        let characters = terminal.cells();

        for (idx, character_cell) in characters.enumerate() {
            if idx % terminal_width == 0 {
                writeln!(self)?;
            }

            let mut character_cell = *character_cell;

            // Protect against alignment issues that can arise from characters
            // like `\0` or `\t` by replacing them with a space.
            //
            // FIXME: Wide characters not handled.
            if character_cell.character.is_whitespace() || character_cell.character.is_control() {
                character_cell.character = ' ';
            }

            write!(self, "{character_cell}")?;
        }

        self.flush()?;

        Ok(())
    }
}

#[cfg(feature = "ansi_terminal")]
impl core::fmt::Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let owo_style: owo_colors::Style = self.style.into();

        write!(f, "{}", self.character.style(owo_style))
    }
}
