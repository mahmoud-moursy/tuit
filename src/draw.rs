//! # The draw module is responsible for the traits that let you draw to the screen
//!
//! Tuit itself does not handle I/O by itself. This means that Tuit users are left responsible
//! for rendering the terminal as well as handling input and output.
//!
//! ## Example
//! ```compile_fail
//! use tuit::errors::Error;
//! use tuit::prelude::*;
//!
//! #[derive(GpuMagic)]
//! struct MyGPU {}
//!
//! impl TerminalDrawTarget for MyGPU {
//!     fn render(&mut self, terminal: impl Terminal) -> Result<(), Error> {
//!         for character in terminal.characters() {
//!             self.gpu_magic(x)
//!         }
//!
//!         Ok(())
//!     }
//! }
//! ```

use crate::terminal::Terminal;

/// This trait is written by the implementor and is responsible for rendering the terminal's data
/// to the screen.
pub trait TerminalDrawTarget {
    /// This method gets called when the implementor calls [`Terminal::display`]
    ///
    /// ## Dummy render implementation:
    ///
    /// ```
    /// use tuit::prelude::*;
    /// use tuit::terminal::TerminalCell;
    /// struct MyGPU;
    ///
    /// impl MyGPU {
    ///     pub fn gpu_magic(&mut self, character: &TerminalCell) {
    ///         // Huzzah!
    ///     }
    /// }
    ///
    /// impl TerminalDrawTarget for MyGPU {
    ///     fn render(&mut self, terminal: impl Terminal) -> tuit::Result<()> {
    ///         let characters = terminal.characters().iter();
    ///
    ///         // Do some magic to render characters!
    ///         for character in characters {
    ///             self.gpu_magic(character)
    ///         }
    ///
    ///         return Ok(())
    ///     }
    /// }
    /// ```
    fn render(&mut self, terminal: impl Terminal) -> crate::Result<()>;
}

#[cfg(feature = "ansi_terminal")]
impl TerminalDrawTarget for std::io::Stdout {
    fn render(&mut self, mut terminal: impl Terminal) -> crate::Result<()> {
        use std::prelude::rust_2021::*;
        use std::io::Write;

        let terminal_width = terminal.width();

        let characters = terminal.characters_mut().iter_mut();

        for (idx, character_cell) in characters.enumerate() {
            if idx % terminal_width == 0 {
                writeln!(self)?
            }
            // Protect against alignment issues that can arise from characters
            // like `\0` or `\t` by replacing them with a space.
            if character_cell.character.is_whitespace() || character_cell.character.is_control() {
                character_cell.character = ' ';
            }

            write!(self, "{character_cell}")?
        }

        Ok(())
    }
}